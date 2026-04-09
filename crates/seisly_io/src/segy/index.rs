use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegyIndex {
    pub inline_range: (i32, i32),
    pub crossline_range: (i32, i32),
    pub sample_count: usize,
    pub sample_interval: f32,
    pub format: u16,
    pub trace_count: usize,
    pub is_regular: bool,
    pub inline_step: i32,
    pub crossline_step: i32,
    // (inline, xline) -> trace_index
    pub trace_map: std::collections::HashMap<(i32, i32), usize>,
}

impl SegyIndex {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index = bincode::deserialize_from(reader)?;
        Ok(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_index_serialization() {
        let mut trace_map = std::collections::HashMap::new();
        trace_map.insert((100, 200), 0);
        
        let index = SegyIndex {
            inline_range: (100, 100),
            crossline_range: (200, 200),
            sample_count: 500,
            sample_interval: 4000.0,
            format: 5,
            trace_count: 1,
            is_regular: true,
            inline_step: 1,
            crossline_step: 1,
            trace_map,
        };

        let tmp = NamedTempFile::new().unwrap();
        index.save(tmp.path()).unwrap();

        let loaded = SegyIndex::load(tmp.path()).unwrap();
        assert_eq!(loaded.inline_range, index.inline_range);
        assert_eq!(loaded.trace_count, index.trace_count);
        assert_eq!(loaded.trace_map.get(&(100, 200)), Some(&0));
    }
}
