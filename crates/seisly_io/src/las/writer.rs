//! LAS (Log ASCII Standard) file writer

use sf_core::domain::well::Well;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LasWriteError {
    #[error("Failed to create LAS file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("No logs to export")]
    NoLogs,
    #[error("Invalid well data: {0}")]
    InvalidData(String),
}

pub struct LasWriter;

impl LasWriter {
    /// Write Well data to LAS 2.0 format
    pub fn write(well: &Well, path: &Path) -> Result<(), LasWriteError> {
        if well.logs.is_empty() {
            return Err(LasWriteError::NoLogs);
        }

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Get depth range from first log
        let first_log = &well.logs[0];
        if first_log.depths.is_empty() {
            return Err(LasWriteError::InvalidData("No depth data".to_string()));
        }

        let min_depth = first_log.min_depth;
        let max_depth = first_log.max_depth;
        let depth_unit = "M";

        // Write Version Section
        writeln!(writer, "~VERSION INFORMATION")?;
        writeln!(
            writer,
            " VERS.                          2.0:   CWLS LOG ASCII STANDARD -VERSION 2.0"
        )?;
        writeln!(
            writer,
            " WRAP.                          NO:   ONE LINE PER DEPTH STEP"
        )?;
        writeln!(writer, " DLM .                          SPACE:   DELIMITER")?;
        writeln!(writer)?;

        // Write Well Section
        writeln!(writer, "~WELL INFORMATION")?;
        writeln!(
            writer,
            " STRT.{}{:12.4}:                :START DEPTH",
            depth_unit, min_depth
        )?;
        writeln!(
            writer,
            " STOP.{}{:12.4}:                :STOP DEPTH",
            depth_unit, max_depth
        )?;
        writeln!(
            writer,
            " STEP.{}{:12.4}:                :STEP",
            depth_unit, 0.5
        )?;
        writeln!(
            writer,
            " NULL.                     -999.25:                :NULL VALUE"
        )?;
        writeln!(
            writer,
            " WELL.                 {:20}:                :WELL NAME",
            well.name
        )?;
        writeln!(
            writer,
            " API .                              :                :API NUMBER"
        )?;
        writeln!(writer)?;

        // Write Curve Section
        writeln!(writer, "~CURVE INFORMATION")?;

        // Depth curve
        writeln!(
            writer,
            " DEPT.{}                      : 1  :DEPTH",
            depth_unit
        )?;

        // Log curves
        for (i, log) in well.logs.iter().enumerate() {
            let desc = if log.description.is_empty() {
                &log.mnemonic
            } else {
                &log.description
            };
            writeln!(
                writer,
                " {:6}.{:8}                  : {:2}  :{}",
                log.mnemonic,
                log.units,
                i + 2,
                desc
            )?;
        }
        writeln!(writer)?;

        // Write Parameter Section (optional)
        writeln!(writer, "~PARAMETER INFORMATION")?;
        writeln!(
            writer,
            " MUD .                              :                :MUD TYPE"
        )?;
        writeln!(
            writer,
            " BHT .                              :                :BOTTOM HOLE TEMP"
        )?;
        writeln!(writer)?;

        // Write Data Section
        writeln!(
            writer,
            "~A  DEPTH     {}",
            well.logs
                .iter()
                .map(|l| &l.mnemonic[..6.min(l.mnemonic.len())])
                .collect::<Vec<_>>()
                .join("        ")
        )?;

        // Write depth and log values
        for depth_idx in 0..first_log.depths.len() {
            write!(writer, "{:9.4}", first_log.depths[depth_idx])?;

            for log in &well.logs {
                if depth_idx < log.data.len() {
                    write!(writer, " {:8.4}", log.data[depth_idx])?;
                } else {
                    write!(writer, "   -999.25")?; // Null value
                }
            }
            writeln!(writer)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Write multiple wells to separate LAS files
    pub fn write_multiple(wells: &[Well], base_path: &Path) -> Result<(), LasWriteError> {
        std::fs::create_dir_all(base_path)?;

        for well in wells {
            let filename = format!("{}.las", well.name.replace(" ", "_"));
            let path = base_path.join(filename);
            Self::write(well, &path)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_core::domain::well::{Well, WellLog};
    use tempfile::TempDir;

    fn create_test_well() -> Well {
        let mut well = Well::new(
            "Test Well".to_string(),
            "TW-1".to_string(),
            500000.0,
            1000000.0,
            50.0,
        );

        // Add GR log
        let mut gr_log = WellLog::new("GR".to_string(), "GAPI".to_string(), Vec::new(), Vec::new());
        gr_log.depths = vec![0.0, 0.5, 1.0, 1.5, 2.0];
        gr_log.data = vec![50.0, 52.0, 48.0, 55.0, 60.0];
        gr_log.min_depth = 0.0;
        gr_log.max_depth = 2.0;
        well.logs.push(gr_log);

        // Add DT log
        let mut dt_log = WellLog::new("DT".to_string(), "US/M".to_string(), Vec::new(), Vec::new());
        dt_log.depths = vec![0.0, 0.5, 1.0, 1.5, 2.0];
        dt_log.data = vec![200.0, 198.0, 202.0, 195.0, 190.0];
        dt_log.min_depth = 0.0;
        dt_log.max_depth = 2.0;
        well.logs.push(dt_log);

        well
    }

    #[test]
    fn test_write_las_file() {
        let well = create_test_well();
        let temp_dir = TempDir::new().unwrap();
        let las_path = temp_dir.path().join("test.las");

        let result = LasWriter::write(&well, &las_path);
        assert!(result.is_ok());
        assert!(las_path.exists());

        // Verify content
        let content = std::fs::read_to_string(&las_path).unwrap();
        assert!(content.contains("~VERSION INFORMATION"));
        assert!(content.contains("~WELL INFORMATION"));
        assert!(content.contains("~CURVE INFORMATION"));
        assert!(content.contains("~A"));
        assert!(content.contains("Test Well"));
        assert!(content.contains("GR"));
        assert!(content.contains("DT"));
    }

    #[test]
    fn test_write_empty_well() {
        let well = Well::new("Empty Well".to_string(), "EW-1".to_string(), 0.0, 0.0, 0.0);
        let temp_dir = TempDir::new().unwrap();
        let las_path = temp_dir.path().join("empty.las");

        let result = LasWriter::write(&well, &las_path);
        assert!(result.is_err());
        assert!(matches!(result, Err(LasWriteError::NoLogs)));
    }
}
