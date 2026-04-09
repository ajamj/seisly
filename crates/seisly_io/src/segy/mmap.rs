use anyhow::Result;
use seisly_core::io::SafeMmap;
use std::fs::File;
use std::path::Path;

use crate::segy::index::SegyIndex;

pub struct MmappedSegy {
    mmap: SafeMmap,
    pub sample_count: usize,
    pub sample_interval: f32,
    pub format: u16,
    pub trace_count: usize,
    // Regular Grid Metadata
    pub is_regular: bool,
    pub inline_step: i32,
    pub crossline_step: i32,
    // Index: (inline, xline) -> trace_index
    index: std::collections::HashMap<(i32, i32), usize>,
    pub inline_range: (i32, i32),
    pub crossline_range: (i32, i32),
}

impl MmappedSegy {
    pub fn new(path: &Path) -> Result<Self> {
        // 1. Try to load sidecar index first
        let index_path = path.with_extension("sfidx");
        if let Ok(loaded_index) = SegyIndex::load(&index_path) {
            log::info!("Loaded sidecar index from {:?}", index_path);
            let file = File::open(path)?;
            let mmap = SafeMmap::map(&file)?;
            
            return Ok(Self {
                mmap,
                sample_count: loaded_index.sample_count,
                sample_interval: loaded_index.sample_interval,
                format: loaded_index.format,
                trace_count: loaded_index.trace_count,
                is_regular: loaded_index.is_regular,
                inline_step: loaded_index.inline_step,
                crossline_step: loaded_index.crossline_step,
                index: loaded_index.trace_map,
                inline_range: loaded_index.inline_range,
                crossline_range: loaded_index.crossline_range,
            });
        }

        let file = File::open(path)?;
        let mmap = SafeMmap::map(&file)?;

        if mmap.len() < 3600 {
            anyhow::bail!("File too small to be a SEG-Y file");
        }

        // Binary header parsing (Big-Endian)
        let sample_interval = mmap.get_u16_be(3216).ok_or_else(|| anyhow::anyhow!("Failed to read sample interval"))? as f32;
        let sample_count = mmap.get_u16_be(3220).ok_or_else(|| anyhow::anyhow!("Failed to read sample count"))? as usize;
        let format = mmap.get_u16_be(3224).ok_or_else(|| anyhow::anyhow!("Failed to read format"))?;

        if sample_count == 0 {
            anyhow::bail!("Sample count is zero in binary header");
        }

        let trace_size = 240 + (sample_count * 4);
        let remaining_size = mmap.len() - 3600;
        let trace_count = remaining_size / trace_size;

        let mut trace_map = std::collections::HashMap::new();
        let mut min_inline = i32::MAX;
        let mut max_inline = i32::MIN;
        let mut min_xline = i32::MAX;
        let mut max_xline = i32::MIN;
        
        let mut prev_iline: Option<i32> = None;
        let mut prev_xline: Option<i32> = None;
        let mut inline_steps = std::collections::HashSet::new();
        let mut crossline_steps = std::collections::HashSet::new();

        // Heuristic: Try to find which header locations actually vary
        for i in 0..trace_count {
            let offset = 3600 + (i * trace_size);
            
            // Standard check
            let iline = mmap.get_i32_be(offset + 188).unwrap_or(0);
            let xline = mmap.get_i32_be(offset + 192).unwrap_or(0);
            
            if iline != 0 || xline != 0 {
                trace_map.insert((iline, xline), i);
                min_inline = min_inline.min(iline);
                max_inline = max_inline.max(iline);
                min_xline = min_xline.min(xline);
                max_xline = max_xline.max(xline);
                
                if let Some(prev) = prev_iline {
                    let diff = (iline - prev).abs();
                    if diff != 0 { inline_steps.insert(diff); }
                }
                if let Some(prev) = prev_xline {
                    let diff = (xline - prev).abs();
                    if diff != 0 { crossline_steps.insert(diff); }
                }
                prev_iline = Some(iline);
                prev_xline = Some(xline);
            }
        }

        // If standard check failed, fallback to linear
        if min_inline == i32::MAX {
            min_inline = 0;
            max_inline = 0;
            min_xline = 0;
            max_xline = trace_count as i32 - 1;
            for i in 0..trace_count {
                trace_map.insert((0, i as i32), i);
            }
        }

        // Regularity check
        let inline_step = inline_steps.into_iter().min().unwrap_or(1);
        let crossline_step = crossline_steps.into_iter().min().unwrap_or(1);
        let is_regular = trace_map.len() == trace_count;

        // Save index for next time
        let index = SegyIndex {
            inline_range: (min_inline, max_inline),
            crossline_range: (min_xline, max_xline),
            sample_count,
            sample_interval,
            format,
            trace_count,
            is_regular,
            inline_step,
            crossline_step,
            trace_map: trace_map.clone(),
        };
        let _ = index.save(&index_path);

        Ok(Self {
            mmap,
            sample_count,
            sample_interval,
            format,
            trace_count,
            is_regular,
            inline_step,
            crossline_step,
            index: trace_map,
            inline_range: (min_inline, max_inline),
            crossline_range: (min_xline, max_xline),
        })
    }

    pub fn read_trace_data(&self, trace_index: usize) -> Option<Vec<f32>> {
        if trace_index >= self.trace_count {
            return None;
        }

        let trace_size = 240 + (self.sample_count * 4);
        let data_offset = 3600 + (trace_index * trace_size) + 240;

        let mut data = Vec::with_capacity(self.sample_count);
        for i in 0..self.sample_count {
            let start = data_offset + (i * 4);

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
        let index = *self.index.get(&(inline, xline))?;
        self.read_trace_data(index)
    }
}

impl seisly_core::seismic::TraceProvider for MmappedSegy {
    fn get_trace(&self, inline: i32, xline: i32) -> Option<Vec<f32>> {
        self.get_trace(inline, xline)
    }

    fn inline_range(&self) -> (i32, i32) {
        self.inline_range
    }

    fn crossline_range(&self) -> (i32, i32) {
        self.crossline_range
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
        assert_eq!(segy.inline_range, (100, 100));
        assert_eq!(segy.crossline_range, (200, 200));

        let trace = segy.get_trace(100, 200).unwrap();
        assert_eq!(trace[0], 1.5);
    }
}
