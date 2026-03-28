use sf_io::segy::mmap::MmappedSegy;

pub trait TraceProvider: Send + Sync {
    fn get_trace(&self, inline: i32, xline: i32) -> Option<Vec<f32>>;
    fn inline_range(&self) -> (i32, i32);
    fn crossline_range(&self) -> (i32, i32);
    fn sample_count(&self) -> usize;
}

pub struct SeismicVolume {
    pub provider: Box<dyn TraceProvider>,
}

impl SeismicVolume {
    pub fn new(provider: Box<dyn TraceProvider>) -> Self {
        Self { provider }
    }

    pub fn get_inline(&self, inline_idx: usize) -> Vec<f32> {
        // inline_idx is 0-based relative to min_inline
        let (min_inline, max_inline) = self.provider.inline_range();
        let (min_xline, max_xline) = self.provider.crossline_range();
        let sample_count = self.provider.sample_count();

        let inline = min_inline + inline_idx as i32;
        if inline > max_inline {
            return Vec::new();
        }

        let xline_count = (max_xline - min_xline + 1) as usize;
        let mut slice = Vec::with_capacity(xline_count * sample_count);

        for xline in min_xline..=max_xline {
            if let Some(trace) = self.provider.get_trace(inline, xline) {
                slice.extend(trace);
            } else {
                // If trace missing, fill with zeros
                slice.extend(vec![0.0; sample_count]);
            }
        }
        slice
    }

    pub fn get_crossline(&self, crossline_idx: usize) -> Vec<f32> {
        // crossline_idx is 0-based relative to min_xline
        let (min_inline, max_inline) = self.provider.inline_range();
        let (min_xline, max_xline) = self.provider.crossline_range();
        let sample_count = self.provider.sample_count();

        let xline = min_xline + crossline_idx as i32;
        if xline > max_xline {
            return Vec::new();
        }

        let inline_count = (max_inline - min_inline + 1) as usize;
        let mut slice = Vec::with_capacity(inline_count * sample_count);

        for inline in min_inline..=max_inline {
            if let Some(trace) = self.provider.get_trace(inline, xline) {
                slice.extend(trace);
            } else {
                slice.extend(vec![0.0; sample_count]);
            }
        }
        slice
    }
}

// Implement for MmappedSegy
impl TraceProvider for MmappedSegy {
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

// In-memory provider for testing and small volumes
pub struct InMemoryProvider {
    pub data: Vec<f32>, // Flat array [inline][xline][sample]
    pub inline_range: (i32, i32),
    pub crossline_range: (i32, i32),
    pub sample_count: usize,
}

impl TraceProvider for InMemoryProvider {
    fn get_trace(&self, inline: i32, xline: i32) -> Option<Vec<f32>> {
        if inline < self.inline_range.0
            || inline > self.inline_range.1
            || xline < self.crossline_range.0
            || xline > self.crossline_range.1
        {
            return None;
        }

        let i_idx = (inline - self.inline_range.0) as usize;
        let x_idx = (xline - self.crossline_range.0) as usize;
        let xline_count = (self.crossline_range.1 - self.crossline_range.0 + 1) as usize;

        let start = (i_idx * xline_count + x_idx) * self.sample_count;
        let end = start + self.sample_count;

        if end <= self.data.len() {
            Some(self.data[start..end].to_vec())
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_inline() {
        let sample_count = 2;
        let inline_range = (0, 1);
        let crossline_range = (0, 1);
        let data = (0..8).map(|x| x as f32).collect(); // 2*2*2 = 8

        let provider = Box::new(InMemoryProvider {
            data,
            inline_range,
            crossline_range,
            sample_count,
        });
        let volume = SeismicVolume::new(provider);

        // Inline 0 should have samples for all height/depth at inline 0
        let inline0 = volume.get_inline(0);
        assert_eq!(inline0, vec![0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_get_crossline() {
        let sample_count = 2;
        let inline_range = (0, 1);
        let crossline_range = (0, 1);
        let data = (0..8).map(|x| x as f32).collect();

        let provider = Box::new(InMemoryProvider {
            data,
            inline_range,
            crossline_range,
            sample_count,
        });
        let volume = SeismicVolume::new(provider);

        // Crossline 0 should have samples for all inlines at crossline 0
        // Trace (0,0) = [0,1], Trace (1,0) = [4,5]
        let xline0 = volume.get_crossline(0);
        assert_eq!(xline0, vec![0.0, 1.0, 4.0, 5.0]);
    }

    #[test]
    fn test_seismic_volume_with_mmap() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmp = NamedTempFile::new().unwrap();

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
        let volume = SeismicVolume::new(Box::new(segy));

        // Querying inline 100, which is relative 0
        let inline0 = volume.get_inline(0);
        // We only have one trace at (100, 200).
        // Inline 100 should contain all xlines in range.
        // Our mmap segy only has one trace, so crossline range is (200, 200).
        // So inline 0 (100) should have one trace (200).
        assert_eq!(inline0.len(), sample_count as usize);
        assert_eq!(inline0[0], 1.5);
    }
}
