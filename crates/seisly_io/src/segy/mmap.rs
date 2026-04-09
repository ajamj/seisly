use anyhow::Result;
use seisly_core::io::SafeMmap;
use std::fs::{self, File};
use std::path::Path;

use crate::segy::index::SegyIndex;

const TEXT_HEADER_SIZE: u64 = 3600;
const TRACE_HEADER_SIZE: u64 = 240;

pub struct MmappedSegy {
    mmap: SafeMmap,
    pub sample_count: usize,
    pub sample_interval: f32,
    pub format: u16,
    pub trace_count: usize,
    pub inline_min: i32,
    pub inline_max: i32,
    pub crossline_min: i32,
    pub crossline_max: i32,
    pub is_regular: bool,
    index: SegyIndex,
}

impl MmappedSegy {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let file_meta = file.metadata()?;
        let segy_mtime = file_meta.modified().ok();

        // 1. Try to load sidecar index (with staleness check)
        let index_path = path.with_extension("sfidx");
        let loaded_index = if let Ok(loaded) = SegyIndex::load(&index_path) {
            // Compare mtimes: if SEG-Y file is newer than the index, rescan.
            let index_meta = fs::metadata(&index_path).ok();
            let index_mtime = index_meta.and_then(|m| m.modified().ok());
            match (segy_mtime, index_mtime) {
                (Some(segy_mt), Some(idx_mt)) if segy_mt > idx_mt => {
                    log::info!(
                        "SEG-Y file is newer than sidecar index; rescanning {:?}",
                        path
                    );
                    None
                }
                _ => Some(loaded),
            }
        } else {
            None
        };

        if let Some(cached) = loaded_index {
            log::info!("Loaded sidecar index from {:?}", index_path);
            let mmap = SafeMmap::map(&file)?;
            return Self::from_index(mmap, cached);
        }

        // 2. Full scan of trace headers
        let mmap = SafeMmap::map(&file)?;
        let scanned = Self::scan_trace_headers(&mmap)?;

        // Save index for next time — log warning but don't fail if save fails
        if let Err(e) = scanned.save(&index_path) {
            log::warn!(
                "Failed to save SEG-Y sidecar index at {}: {} — next load will rescan",
                index_path.display(),
                e
            );
        }

        Self::from_index(mmap, scanned)
    }

    /// Build an MmappedSegy from a pre-computed SegyIndex.
    fn from_index(mmap: SafeMmap, index: SegyIndex) -> Result<Self> {
        Ok(Self {
            inline_min: index.inline_min,
            inline_max: index.inline_max,
            crossline_min: index.crossline_min,
            crossline_max: index.crossline_max,
            sample_count: index.sample_count,
            sample_interval: index.sample_interval,
            format: index.format,
            trace_count: index.trace_count,
            is_regular: index.is_regular,
            index,
            mmap,
        })
    }

    /// Scan all trace headers to build a SegyIndex.
    /// This extracts the logic previously inlined in `new()`.
    pub fn scan_trace_headers(mmap: &SafeMmap) -> Result<SegyIndex> {
        if mmap.len() < TEXT_HEADER_SIZE as usize {
            anyhow::bail!("File too small to be a SEG-Y file");
        }

        let sample_interval = mmap
            .get_u16_be(3216)
            .ok_or_else(|| anyhow::anyhow!("Failed to read sample interval"))?
            as f32;
        let sample_count = mmap
            .get_u16_be(3220)
            .ok_or_else(|| anyhow::anyhow!("Failed to read sample count"))?
            as usize;
        let format = mmap
            .get_u16_be(3224)
            .ok_or_else(|| anyhow::anyhow!("Failed to read format"))?;

        if sample_count == 0 {
            anyhow::bail!("Sample count is zero in binary header");
        }

        let trace_data_size = sample_count * 4;
        let trace_size = TRACE_HEADER_SIZE as usize + trace_data_size;
        let remaining_size = mmap.len() - TEXT_HEADER_SIZE as usize;
        let trace_count = remaining_size / trace_size;

        let mut trace_byte_offsets = Vec::with_capacity(trace_count);
        let mut min_inline = i32::MAX;
        let mut max_inline = i32::MIN;
        let mut min_xline = i32::MAX;
        let mut max_xline = i32::MIN;

        let mut prev_iline: Option<i32> = None;
        let mut prev_xline: Option<i32> = None;
        let mut inline_steps = std::collections::HashSet::new();
        let mut crossline_steps = std::collections::HashSet::new();

        for i in 0..trace_count {
            let header_offset = TEXT_HEADER_SIZE as usize + i * trace_size;

            let iline = mmap.get_i32_be(header_offset + 188).unwrap_or(0);
            let xline = mmap.get_i32_be(header_offset + 192).unwrap_or(0);

            let data_offset = (header_offset + TRACE_HEADER_SIZE as usize) as u64;
            trace_byte_offsets.push(data_offset);

            if iline != 0 || xline != 0 {
                min_inline = min_inline.min(iline);
                max_inline = max_inline.max(iline);
                min_xline = min_xline.min(xline);
                max_xline = max_xline.max(xline);

                if let Some(prev) = prev_iline {
                    let diff = (iline - prev).abs();
                    if diff != 0 {
                        inline_steps.insert(diff);
                    }
                }
                if let Some(prev) = prev_xline {
                    let diff = (xline - prev).abs();
                    if diff != 0 {
                        crossline_steps.insert(diff);
                    }
                }
                prev_iline = Some(iline);
                prev_xline = Some(xline);
            }
        }

        // If standard check found nothing, fall back to linear indexing.
        if min_inline == i32::MAX {
            min_inline = 0;
            max_inline = 0;
            min_xline = 0;
            max_xline = trace_count as i32 - 1;
        }

        // Grid is regular only if there's at most one unique step size in each direction.
        // Single-trace files (0 steps) are considered regular by default.
        let inline_step_count = inline_steps.len();
        let crossline_step_count = crossline_steps.len();
        let crossline_step = crossline_steps.into_iter().min().unwrap_or(1);
        let is_regular = inline_step_count <= 1 && crossline_step_count <= 1;

        // Compute strides for regular grids.
        // For a regular grid: trace_index = (il - inline_min) * n_crosslines + (xl - crossline_min)
        // So inline_stride = n_crosslines, crossline_stride = 1.
        let (inline_stride, crossline_stride) = if is_regular && crossline_step > 0 {
            let n_crosslines = ((max_xline - min_xline) / crossline_step + 1) as u64;
            if n_crosslines > 0 {
                (Some(n_crosslines), Some(1u64))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        Ok(SegyIndex {
            inline_min: min_inline,
            inline_max: max_inline,
            crossline_min: min_xline,
            crossline_max: max_xline,
            trace_count,
            sample_count,
            sample_interval,
            format,
            is_regular,
            trace_byte_offsets,
            inline_stride,
            crossline_stride,
            trace_header_size: TRACE_HEADER_SIZE,
            trace_data_offset: TEXT_HEADER_SIZE,
        })
    }

    /// Compute the byte offset for a given (inline, crossline) pair in O(1) for regular grids.
    /// Returns None if the grid is irregular or the coordinates are out of range.
    pub fn get_trace_offset(&self, inline: i32, xline: i32) -> Option<u64> {
        if self.is_regular {
            let il_stride = self.index.inline_stride?;
            let xl_stride = self.index.crossline_stride?;
            // Use checked_sub to prevent overflow panic on i32::MIN - i32::MAX
            let il_diff = inline.checked_sub(self.inline_min)?;
            let xl_diff = xline.checked_sub(self.crossline_min)?;
            // Use checked multiplication with i64 to prevent overflow
            let il_offset = (il_diff as i64).checked_mul(il_stride as i64)?;
            let xl_offset = (xl_diff as i64).checked_mul(xl_stride as i64)?;
            let trace_index = il_offset.checked_add(xl_offset)? as u64;
            if trace_index >= self.trace_count as u64 {
                return None;
            }
            return self.index.trace_data_offset.checked_add(
                trace_index * (self.index.trace_header_size + (self.sample_count as u64) * 4)
                    + self.index.trace_header_size,
            );
        }
        // Fallback: linear scan via the stored offsets (irregular grids).
        // For irregular grids, we store byte offsets directly.
        if inline >= self.inline_min
            && inline <= self.inline_max
            && xline >= self.crossline_min
            && xline <= self.crossline_max
        {
            // We don't have a direct (il, xl) -> index mapping in the new schema.
            // The caller should use read_trace_data by index for irregular grids.
            None
        } else {
            None
        }
    }

    pub fn read_trace_data(&self, trace_index: usize) -> Option<Vec<f32>> {
        if trace_index >= self.trace_count {
            return None;
        }

        // Use pre-computed byte offset from the index.
        let data_offset = if trace_index < self.index.trace_byte_offsets.len() {
            self.index.trace_byte_offsets[trace_index] as usize
        } else {
            // Fallback computation.
            let trace_size = TRACE_HEADER_SIZE as usize + self.sample_count * 4;
            TEXT_HEADER_SIZE as usize + trace_index * trace_size + TRACE_HEADER_SIZE as usize
        };

        let mut data = Vec::with_capacity(self.sample_count);
        for i in 0..self.sample_count {
            let start = data_offset + i * 4;

            let val = if self.format == 5 {
                self.mmap.get_f32_be(start)?
            } else if self.format == 1 {
                let bytes = self.mmap.get_u32_be(start)?;
                ibm_to_ieee_f32(bytes)
            } else {
                0.0
            };
            data.push(val);
        }

        Some(data)
    }

    pub fn get_trace(&self, inline: i32, xline: i32) -> Option<Vec<f32>> {
        // Try O(1) offset computation for regular grids.
        if let Some(offset) = self.get_trace_offset(inline, xline) {
            let mut data = Vec::with_capacity(self.sample_count);
            for i in 0..self.sample_count {
                let start = offset as usize + i * 4;
                let val = if self.format == 5 {
                    self.mmap.get_f32_be(start)?
                } else if self.format == 1 {
                    let bytes = self.mmap.get_u32_be(start)?;
                    ibm_to_ieee_f32(bytes)
                } else {
                    0.0
                };
                data.push(val);
            }
            return Some(data);
        }
        None
    }

    /// Returns the index of a trace by (inline, crossline), if it can be resolved.
    pub fn get_trace_index(&self, inline: i32, xline: i32) -> Option<usize> {
        if self.is_regular {
            let il_stride = self.index.inline_stride?;
            let xl_stride = self.index.crossline_stride?;
            let il_offset = (inline - self.inline_min).checked_mul(il_stride as i32)?;
            let xl_offset = (xline - self.crossline_min).checked_mul(xl_stride as i32)?;
            let idx = il_offset.checked_add(xl_offset)?;
            if idx < 0 || idx as usize >= self.trace_count {
                return None;
            }
            Some(idx as usize)
        } else {
            // For irregular grids, scan for the matching coordinates.
            for (i, &offset) in self.index.trace_byte_offsets.iter().enumerate() {
                let header_offset = offset as usize - TRACE_HEADER_SIZE as usize;
                let il = self.mmap.get_i32_be(header_offset + 188)?;
                let xl = self.mmap.get_i32_be(header_offset + 192)?;
                if il == inline && xl == xline {
                    return Some(i);
                }
            }
            None
        }
    }
}

impl seisly_core::seismic::TraceProvider for MmappedSegy {
    fn get_trace(&self, inline: i32, xline: i32) -> Option<Vec<f32>> {
        self.get_trace(inline, xline)
    }

    fn inline_range(&self) -> (i32, i32) {
        (self.inline_min, self.inline_max)
    }

    fn crossline_range(&self) -> (i32, i32) {
        (self.crossline_min, self.crossline_max)
    }

    fn sample_count(&self) -> usize {
        self.sample_count
    }
}

/// Simple IBM to IEEE float conversion
fn ibm_to_ieee_f32(ibm: u32) -> f32 {
    if ibm == 0 {
        return 0.0;
    }
    let sign = (ibm >> 31) & 0x01;
    let exponent = (ibm >> 24) & 0x7F;
    let fraction = (ibm & 0x00FFFFFF) as f32 / 16777216.0;

    let sign_f = if sign == 1 { -1.0 } else { 1.0 };
    sign_f * fraction * 16.0f32.powi(exponent as i32 - 64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_mmap_segy_indexing() {
        let mut tmp = NamedTempFile::new().unwrap();

        // 3200 text + 400 binary + 1 trace (240 head + 10 samples * 4)
        let sample_count = 10u16;
        let mut content = vec![0u8; 3600];

        // Binary header
        content[3216] = 0x0F;
        content[3217] = 0xA0; // interval 4000
        content[3220] = 0x00;
        content[3221] = sample_count as u8; // count 10
        content[3224] = 0x00;
        content[3225] = 0x05; // format 5 (IEEE)

        // Trace 1: Inline 100, Xline 200
        let mut trace1_head = vec![0u8; 240];
        let iline = 100i32.to_be_bytes();
        let xline = 200i32.to_be_bytes();
        trace1_head[188..192].copy_from_slice(&iline);
        trace1_head[192..196].copy_from_slice(&xline);

        let mut trace1_data = vec![0u8; sample_count as usize * 4];
        let val = 1.5f32.to_be_bytes();
        trace1_data[0..4].copy_from_slice(&val);

        content.extend_from_slice(&trace1_head);
        content.extend_from_slice(&trace1_data);

        tmp.write_all(&content).unwrap();

        let segy = MmappedSegy::new(tmp.path()).unwrap();
        assert_eq!(segy.trace_count, 1);
        assert_eq!(segy.inline_min, 100);
        assert_eq!(segy.inline_max, 100);
        assert_eq!(segy.crossline_min, 200);
        assert_eq!(segy.crossline_max, 200);

        let trace = segy.get_trace(100, 200).unwrap();
        assert_eq!(trace[0], 1.5);
    }

    #[test]
    fn test_scan_trace_headers() {
        let mut tmp = NamedTempFile::new().unwrap();

        let sample_count = 5u16;
        let mut content = vec![0u8; 3600];
        content[3216] = 0x0F;
        content[3217] = 0xA0;
        content[3220] = 0x00;
        content[3221] = sample_count as u8;
        content[3224] = 0x00;
        content[3225] = 0x05;

        // Write 2 traces
        for (_i, (il, xl)) in [(10i32, 20i32), (10i32, 21i32)].iter().enumerate() {
            let mut head = vec![0u8; 240];
            head[188..192].copy_from_slice(&il.to_be_bytes());
            head[192..196].copy_from_slice(&xl.to_be_bytes());
            content.extend_from_slice(&head);
            content.extend_from_slice(&vec![0u8; sample_count as usize * 4]);
        }

        tmp.write_all(&content).unwrap();

        let file = File::open(tmp.path()).unwrap();
        let mmap = SafeMmap::map(&file).unwrap();
        let index = MmappedSegy::scan_trace_headers(&mmap).unwrap();

        assert_eq!(index.trace_count, 2);
        assert_eq!(index.inline_min, 10);
        assert_eq!(index.inline_max, 10);
        assert_eq!(index.crossline_min, 20);
        assert_eq!(index.crossline_max, 21);
        assert_eq!(index.sample_count, 5);
        assert_eq!(index.trace_byte_offsets.len(), 2);
    }

    #[test]
    fn test_get_trace_offset_regular() {
        let mut tmp = NamedTempFile::new().unwrap();

        let sample_count = 4u16;
        let mut content = vec![0u8; 3600];
        content[3216] = 0x0F;
        content[3217] = 0xA0;
        content[3220] = 0x00;
        content[3221] = sample_count as u8;
        content[3224] = 0x00;
        content[3225] = 0x05;

        // 2x2 regular grid: il=100..101, xl=200..201
        for il in 100..=101i32 {
            for xl in 200..=201i32 {
                let mut head = vec![0u8; 240];
                head[188..192].copy_from_slice(&il.to_be_bytes());
                head[192..196].copy_from_slice(&xl.to_be_bytes());
                content.extend_from_slice(&head);
                content.extend_from_slice(&vec![0u8; sample_count as usize * 4]);
            }
        }

        tmp.write_all(&content).unwrap();

        let segy = MmappedSegy::new(tmp.path()).unwrap();
        assert!(segy.is_regular);

        // Offsets should be valid for the regular grid
        assert!(segy.get_trace_offset(100, 200).is_some());
        assert!(segy.get_trace_offset(100, 201).is_some());
        assert!(segy.get_trace_offset(101, 200).is_some());
        assert!(segy.get_trace_offset(101, 201).is_some());

        // Out of range
        assert!(segy.get_trace_offset(99, 200).is_none());
        assert!(segy.get_trace_offset(102, 200).is_none());
    }
}
