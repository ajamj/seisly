use anyhow::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub struct SegyMetadata {
    pub inline_range: (i32, i32),
    pub crossline_range: (i32, i32),
    pub sample_count: usize,
    pub sample_interval: f32,
    pub format: u16, // 1: IBM float, 5: IEEE float
}

pub fn parse_metadata(path: &Path) -> Result<SegyMetadata> {
    let mut file = File::open(path)?;

    // Skip 3200-byte EBCDIC/ASCII header
    file.seek(SeekFrom::Start(3200))?;

    // Read 400-byte binary header
    let mut binary_header = [0u8; 400];
    file.read_exact(&mut binary_header)?;

    // Extract key fields (Big-Endian)
    let sample_interval = u16::from_be_bytes([binary_header[16], binary_header[17]]) as f32;
    let sample_count = u16::from_be_bytes([binary_header[20], binary_header[21]]) as usize;
    let format = u16::from_be_bytes([binary_header[24], binary_header[25]]);

    // Determine ranges (requires scanning headers, so placeholder for now until Task 3)
    // Actually, task 2 says "Implement basic header reading logic"
    // Scanning all trace headers to find ranges is expensive and should probably be done once or during mmap build.

    Ok(SegyMetadata {
        inline_range: (1, 1),    // Placeholder: to be refined in Task 3
        crossline_range: (1, 1), // Placeholder: to be refined in Task 3
        sample_count,
        sample_interval,
        format,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_binary_header() {
        let mut tmp = NamedTempFile::new().unwrap();

        // Write mock 3200-byte header
        let text_header = [0u8; 3200];
        tmp.write_all(&text_header).unwrap();

        // Write mock 400-byte binary header
        let mut binary_header = [0u8; 400];
        // Sample interval: 4000 (at bytes 16-17)
        binary_header[16] = 0x0F;
        binary_header[17] = 0xA0;
        // Sample count: 500 (at bytes 20-21)
        binary_header[20] = 0x01;
        binary_header[21] = 0xF4;
        // Format: 5 (at bytes 24-25)
        binary_header[25] = 0x05;

        tmp.write_all(&binary_header).unwrap();

        let metadata = parse_metadata(tmp.path()).unwrap();
        assert_eq!(metadata.sample_count, 500);
        assert_eq!(metadata.sample_interval, 4000.0);
        assert_eq!(metadata.format, 5);
    }
}
