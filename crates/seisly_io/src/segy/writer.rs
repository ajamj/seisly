//! SEG-Y file writer
//!
//! Supports:
//! - Custom binary header parameters
//! - Trace-by-trace writing
//! - Standard SEG-Y rev 2.0 format

use giga_segy_out::{
    create_headers::{CreateBinHeader, CreateTraceHeader},
    enums::{FixedLengthTraces, SampleFormatCode},
    BinHeader, SegyFile, SegySettings, TraceHeader,
};
use std::fs::File;
use std::path::Path;

use crate::segy::reader::IoError;

/// SEG-Y volume writer
pub struct SegyWriter {
    segy: SegyFile<SegySettings, File>,
    trace_count: u32,
    samples_per_trace: u32,
    next_trace_index: u32,
}

impl SegyWriter {
    /// Create a new SEG-Y writer
    ///
    /// # Arguments
    /// * `path` - Output file path
    /// * `sample_rate` - Sample interval in microseconds (e.g., 4000 = 4ms)
    /// * `trace_count` - Total number of traces to write
    /// * `samples_per_trace` - Number of samples per trace
    pub fn new<P: AsRef<Path>>(
        path: P,
        sample_rate: u32,
        trace_count: u32,
        samples_per_trace: u32,
    ) -> Result<Self, IoError> {
        // Validate parameters
        if trace_count == 0 {
            return Err(IoError::ParseError(
                "Trace count must be greater than 0".to_string(),
            ));
        }
        if samples_per_trace == 0 {
            return Err(IoError::ParseError(
                "Samples per trace must be greater than 0".to_string(),
            ));
        }
        if sample_rate == 0 {
            return Err(IoError::ParseError(
                "Sample rate must be greater than 0".to_string(),
            ));
        }

        // Create binary header
        let mut bin_header = BinHeader::default();
        bin_header.no_traces = trace_count as u16;
        bin_header.sample_interval = (sample_rate as u16).max(1);
        bin_header.no_samples = (samples_per_trace as u16).max(1);
        bin_header.sample_format_code = SampleFormatCode::Float32;
        bin_header.fixed_length_trace_flag = FixedLengthTraces::Yes;

        // Create text header (3200 bytes, standard SEG-Y format)
        let text_header = create_default_text_header();

        // Create the SEG-Y file
        let segy = SegyFile::<SegySettings, File>::create_file(
            path.as_ref(),
            SegySettings::default(),
            text_header,
            bin_header,
            None, // No tape label
        )
        .map_err(|e| IoError::ParseError(format!("Failed to create SEG-Y file: {}", e)))?;

        Ok(Self {
            segy,
            trace_count,
            samples_per_trace,
            next_trace_index: 0,
        })
    }

    /// Write a trace at specified index
    ///
    /// # Arguments
    /// * `index` - Trace index (0-based)
    /// * `data` - Trace data as f32 slice
    pub fn write_trace(&mut self, index: u32, data: &[f32]) -> Result<(), IoError> {
        if index >= self.trace_count {
            return Err(IoError::ParseError(format!(
                "Trace index {} out of bounds (max {})",
                index,
                self.trace_count - 1
            )));
        }

        if data.len() != self.samples_per_trace as usize {
            return Err(IoError::ParseError(format!(
                "Trace data length {} doesn't match expected {}",
                data.len(),
                self.samples_per_trace
            )));
        }

        // Create trace header for 2D seismic data
        // Using new_2d which sets: x_ensemble, y_ensemble, coordinate_scalar
        let trace_header = TraceHeader::new_2d(
            (index + 1) as i32,
            (index + 1) as i32,
            0,
        );

        // Write trace data (lossless since we're using f32)
        self.segy
            .add_trace_lossless(trace_header, None, data.to_vec())
            .map_err(|e| IoError::ParseError(format!("Failed to write trace: {}", e)))?;

        self.next_trace_index = index + 1;
        Ok(())
    }

    /// Finish writing and flush
    pub fn finish(self) -> Result<(), IoError> {
        // The SegyFile is dropped here, which finalizes the file
        // No explicit finish method needed in giga-segy-out
        Ok(())
    }
}

/// Create a default 3200-byte text header
fn create_default_text_header() -> String {
    // Standard SEG-Y text header format
    // 80 bytes per line, 40 lines = 3200 bytes
    let mut header = String::with_capacity(3200);

    // Line 1: Client and company identification
    header.push_str(&format!("{:<80}", "CLIENT: STRATAFORGE PRO"));
    header.push_str(&format!("{:<80}", "COMPANY: STRATAFORGE"));

    // Line 3: File identification
    header.push_str(&format!("{:<80}", "FILE: SEG-Y REV 2.0"));

    // Line 4: Line identification
    header.push_str(&format!("{:<80}", "LINE: DEFAULT"));

    // Line 5: Reel number
    header.push_str(&format!("{:<80}", "REEL: 001"));

    // Line 6: Channel/ensemble details
    header.push_str(&format!("{:<80}", "CHANNEL: 1 ENSEMBLE: 1"));

    // Line 7: Data type
    header.push_str(&format!("{:<80}", "DATA TYPE: SEISMIC"));

    // Line 8: Binary gain
    header.push_str(&format!("{:<80}", "BINARY GAIN: FIXED"));

    // Line 9: Amplitude recovery
    header.push_str(&format!("{:<80}", "AMPLITUDE RECOVERY: NONE"));

    // Line 10: Measurement system
    header.push_str(&format!("{:<80}", "MEASUREMENT SYSTEM: METERS"));

    // Fill remaining lines with spaces to reach 3200 bytes
    while header.len() < 3200 {
        header.push(' ');
    }

    // Ensure exactly 3200 bytes
    header.truncate(3200);
    header
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_segy_writer_new() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().join("test.segy");
        let result = SegyWriter::new(&temp_path, 4000, 10, 100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_segy_writer_error_on_zero_trace_count() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().join("test.segy");
        let result = SegyWriter::new(&temp_path, 4000, 0, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_segy_writer_error_on_zero_samples() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().join("test.segy");
        let result = SegyWriter::new(&temp_path, 4000, 10, 0);
        assert!(result.is_err());
    }
}
