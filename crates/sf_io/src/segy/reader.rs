//! SEG-Y file reader with memory-mapped access
//!
//! Hybrid approach:
//! - giga-segy-in: Header parsing (EBCDIC decoding, binary header)
//! - memmap2: Zero-copy trace data access
//! - Custom: Optimized I/O layer

use giga_segy_in::{SegyFile, SegySettings};
use std::fs::File;
use std::path::Path;
use thiserror::Error;

/// IO error type for SEG-Y operations
#[derive(Error, Debug)]
pub enum IoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// SEG-Y volume reader
pub struct SegyReader {
    segy: SegyFile,
}

impl SegyReader {
    /// Open a SEG-Y file with memory mapping
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
        let file = File::open(path.as_ref())?;

        // Use default settings for standard SEG-Y format
        let settings = SegySettings::default();
        let segy = SegyFile::open(file, settings)
            .map_err(|e| IoError::ParseError(format!("Failed to parse SEG-Y file: {}", e)))?;

        Ok(Self { segy })
    }

    /// Get textual header (EBCDIC/ASCII)
    pub fn textual_header(&self) -> &str {
        self.segy.get_text_header()
    }

    /// Get binary header
    pub fn binary_header(&self) -> &giga_segy_in::BinHeader {
        self.segy.get_bin_header()
    }

    /// Get number of traces
    pub fn trace_count(&self) -> usize {
        self.segy.traces_iter().count()
    }

    /// Read a trace by index
    pub fn read_trace(&self, index: usize) -> Result<Vec<f32>, IoError> {
        let trace = self.segy
            .traces_iter()
            .nth(index)
            .ok_or_else(|| IoError::ParseError(format!("Trace index {} out of range", index)))?;

        // Extract trace data as f32
        let data = self.segy
            .get_trace_data_as_f32_from_trace(&trace)
            .map_err(|e| IoError::ParseError(format!("Failed to read trace data: {}", e)))?;

        Ok(data)
    }

    /// Get trace header at index
    pub fn trace_header(&self, _index: usize) -> Result<giga_segy_in::TraceHeader, IoError> {
        // Note: Direct access to trace header is not exposed in giga-segy-in 0.5
        // This is a limitation of the current API
        Err(IoError::ParseError(
            "trace_header access not supported in this version".to_string(),
        ))
    }
}

/// Extended binary header with convenient accessors
pub struct ExtendedBinaryHeader {
    pub sample_rate: u32,
    pub trace_count: u32,
    pub samples_per_trace: u32,
    pub data_format: u16,
}

impl From<&giga_segy_in::BinHeader> for ExtendedBinaryHeader {
    fn from(header: &giga_segy_in::BinHeader) -> Self {
        Self {
            sample_rate: header.sample_interval as u32,
            trace_count: header.no_traces as u32,
            samples_per_trace: header.no_samples as u32,
            data_format: header.sample_format_code as u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segy_reader_error_on_missing_file() {
        let result = SegyReader::open("non_existent.segy");
        assert!(result.is_err());
    }
}
