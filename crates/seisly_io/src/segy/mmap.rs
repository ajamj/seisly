use anyhow::Result;
use memmap2::Mmap;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub struct MmappedSegy {
    mmap: Mmap,
    pub sample_count: usize,
    pub sample_interval: f32,
    pub format: u16,
    pub trace_count: usize,
    // Index: (inline, xline) -> trace_index
    index: HashMap<(i32, i32), usize>,
    pub inline_range: (i32, i32),
    pub crossline_range: (i32, i32),
}

impl MmappedSegy {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < 3600 {
            anyhow::bail!("File too small to be a SEG-Y file");
        }

        // Binary header parsing (Big-Endian)
        let sample_interval = u16::from_be_bytes([mmap[3216], mmap[3217]]) as f32;
        let sample_count = u16::from_be_bytes([mmap[3220], mmap[3221]]) as usize;
        let format = u16::from_be_bytes([mmap[3224], mmap[3225]]);

        if sample_count == 0 {
            anyhow::bail!("Sample count is zero in binary header");
        }

        let trace_size = 240 + (sample_count * 4);
        let remaining_size = mmap.len() - 3600;
        let trace_count = remaining_size / trace_size;

        let mut index = HashMap::new();
        let mut min_inline = i32::MAX;
        let mut max_inline = i32::MIN;
        let mut min_xline = i32::MAX;
        let mut max_xline = i32::MIN;

        for i in 0..trace_count {
            let offset = 3600 + (i * trace_size);

            // Standard SEG-Y Inline/Crossline locations: 189 and 193 (0-based: 188 and 192)
            let iline = i32::from_be_bytes([
                mmap[offset + 188],
                mmap[offset + 189],
                mmap[offset + 190],
                mmap[offset + 191],
            ]);
            let xline = i32::from_be_bytes([
                mmap[offset + 192],
                mmap[offset + 193],
                mmap[offset + 194],
                mmap[offset + 195],
            ]);

            index.insert((iline, xline), i);

            min_inline = min_inline.min(iline);
            max_inline = max_inline.max(iline);
            min_xline = min_xline.min(xline);
            max_xline = max_xline.max(xline);
        }

        Ok(Self {
            mmap,
            sample_count,
            sample_interval,
            format,
            trace_count,
            index,
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
            let bytes = [
                self.mmap[start],
                self.mmap[start + 1],
                self.mmap[start + 2],
                self.mmap[start + 3],
            ];

            let val = if self.format == 5 {
                f32::from_be_bytes(bytes)
            } else if self.format == 1 {
                // IBM Float placeholder - common in legacy SEG-Y
                ibm_to_ieee_f32(u32::from_be_bytes(bytes))
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
