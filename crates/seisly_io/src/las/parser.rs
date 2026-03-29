//! LAS (Log ASCII Standard) file parser

use sf_core::domain::well::{Well, WellLog};
use std::io::{BufRead, BufReader};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LasError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid LAS version: {0}")]
    InvalidVersion(String),
    #[error("Missing required section: {0}")]
    MissingSection(String),
    #[error("No data section found")]
    NoDataSection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LasVersion {
    Las20,
    Las30,
    Unknown,
}

/// Curve definition from LAS header
#[derive(Debug, Clone)]
pub struct CurveDef {
    pub mnemonic: String,
    pub unit: String,
    pub description: String,
}

pub struct LasParser;

impl LasParser {
    /// Read LAS file and convert to Well model
    pub fn read(path: &Path) -> Result<Well, LasError> {
        // First pass: detect version
        let version = Self::detect_version(path)?;

        // Second pass: parse file
        let file = std::fs::File::open(path)
            .map_err(|_| LasError::FileNotFound(path.display().to_string()))?;
        let reader = BufReader::new(file);

        match version {
            LasVersion::Las20 => Self::parse_las_20(reader, path),
            LasVersion::Las30 => Err(LasError::InvalidVersion(
                "LAS 3.0 not yet implemented".to_string(),
            )),
            LasVersion::Unknown => Err(LasError::InvalidVersion("Unknown LAS version".to_string())),
        }
    }

    /// Detect LAS version from file
    fn detect_version(path: &Path) -> Result<LasVersion, LasError> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| LasError::FileNotFound(path.display().to_string()))?;

        for line in content.lines() {
            let trimmed = line.trim();

            // Look for VERS. in any section
            if trimmed.starts_with("VERS.") {
                if trimmed.contains("2.0") {
                    return Ok(LasVersion::Las20);
                } else if trimmed.contains("3.0") {
                    return Ok(LasVersion::Las30);
                }
            }
        }
        Ok(LasVersion::Unknown)
    }

    /// Parse LAS 2.0 format
    fn parse_las_20<R: BufRead>(reader: R, _path: &Path) -> Result<Well, LasError> {
        let mut well_name = String::new();
        let mut data_lines = Vec::new();
        let mut section = "OTHER";
        let mut null_value = -999.25f32;
        let mut curve_defs: Vec<CurveDef> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| LasError::ParseError(e.to_string()))?;
            let trimmed = line.trim();

            if trimmed.starts_with('~') {
                section = if trimmed.starts_with("~WELL") {
                    "WELL"
                } else if trimmed.starts_with("~CURVE") {
                    "CURVE"
                } else if trimmed.starts_with("~PARAMETER") || trimmed.starts_with("~PARAM") {
                    "PARAM"
                } else if trimmed.starts_with("~A") || trimmed.starts_with("~ASCII") {
                    "DATA"
                } else {
                    "OTHER"
                };
                continue;
            }

            if section == "PARAM" && trimmed.starts_with(" NULL.") {
                null_value = Self::extract_numeric_value(trimmed).unwrap_or(-999.25);
            }

            if section == "WELL" && trimmed.starts_with(" WELL.") {
                well_name = Self::extract_value(trimmed).unwrap_or_default();
            }

            if section == "CURVE" && !trimmed.is_empty() {
                if let Some(curve_def) = Self::parse_curve_def(trimmed) {
                    curve_defs.push(curve_def);
                }
            }

            if section == "DATA" && !trimmed.is_empty() {
                data_lines.push(trimmed.to_string());
            }
        }

        if data_lines.is_empty() {
            return Err(LasError::NoDataSection);
        }

        // Parse data and create logs
        let mut logs = Vec::new();
        let mut all_depths = Vec::new();

        for line in &data_lines {
            let values: Vec<f32> = line
                .split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect();

            if values.is_empty() {
                continue;
            }

            all_depths.push(values[0]);

            // Process curve values (skip depth which is first)
            for (i, &val) in values.iter().enumerate().skip(1) {
                if val == null_value {
                    continue; // Skip null values
                }

                if i > logs.len() {
                    // Create new log
                    let (mnemonic, unit) = if i - 1 < curve_defs.len() {
                        (
                            curve_defs[i - 1].mnemonic.clone(),
                            curve_defs[i - 1].unit.clone(),
                        )
                    } else {
                        (format!("CURVE_{}", i), "UNKNOWN".to_string())
                    };

                    let mut log = WellLog::new(mnemonic, unit, Vec::new(), Vec::new());
                    log.data.push(val);
                    logs.push(log);
                } else {
                    logs[i - 1].data.push(val);
                }
            }
        }

        // Set depths for all logs
        for log in &mut logs {
            log.depths = all_depths.clone();
            if !log.depths.is_empty() {
                log.min_depth = *log
                    .depths
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0.0);
                log.max_depth = *log
                    .depths
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0.0);
            }
        }

        // Create Well model
        let mut well = Well::new(
            well_name.clone(),
            well_name.clone(),
            0.0, // X coordinate (not in LAS 2.0, set to 0)
            0.0, // Y coordinate (not in LAS 2.0, set to 0)
            0.0, // Elevation (not in LAS 2.0, set to 0)
        );
        well.logs = logs;

        Ok(well)
    }

    /// Parse a curve definition line
    fn parse_curve_def(line: &str) -> Option<CurveDef> {
        if line.starts_with('~') || line.is_empty() {
            return None;
        }

        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() < 2 {
            return None;
        }

        let mnemonic_unit = parts[0].trim();
        let description = parts[1].trim();

        // Parse mnemonic and unit (e.g., "DEPT.M")
        if let Some(dot_pos) = mnemonic_unit.find('.') {
            let mnemonic = mnemonic_unit[..dot_pos].trim().to_string();
            let unit = mnemonic_unit[dot_pos + 1..]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();

            // Skip if mnemonic looks like a section header
            if mnemonic.len() > 10 || mnemonic.is_empty() {
                return None;
            }

            return Some(CurveDef {
                mnemonic,
                unit,
                description: description.to_string(),
            });
        }

        None
    }

    /// Extract numeric value from LAS line
    fn extract_numeric_value(line: &str) -> Option<f32> {
        line.split_whitespace()
            .nth(1)
            .and_then(|v| v.split('.').next())
            .and_then(|v| v.parse().ok())
    }

    /// Extract string value from LAS line
    fn extract_value(line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() > 1 {
            let value_part = parts[1];
            let value = value_part.split('.').next().unwrap_or("").trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_las(content: &str) -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let las_path = temp_dir.path().join("test.las");
        let mut file = std::fs::File::create(&las_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        temp_dir
    }

    #[test]
    fn test_parse_simple_las() {
        let las_content = "~VERSION INFORMATION
 VERS.                          2.0 :   CWLS LOG ASCII STANDARD -VERSION 2.0
 WRAP.                           NO :   ONE LINE PER DEPTH STEP
~WELL INFORMATION
 STRT.M                       100.0 : START DEPTH
 STOP.M                       102.0 : STOP DEPTH
 STEP.M                         0.5 : STEP
 NULL.                        -999.25 : NULL VALUE
 WELL.                      WELL-1 : WELL NAME
~CURVE INFORMATION
 DEPT.M                      : 1  DEPTH
 GR.GAPI                   : 2  GAMMA RAY
 RES.OHMM                  : 3  RESISTIVITY
~A  DEPTH     GR    RES
 100.0   50.5  10.2
 100.5   51.0  10.5
 101.0   52.3  10.8
";
        let temp_dir = create_test_las(las_content);
        let las_path = temp_dir.path().join("test.las");

        let well = LasParser::read(&las_path).unwrap();
        assert!(!well.logs.is_empty());
        assert!(well.logs[0].data.len() >= 2);
    }

    #[test]
    fn test_parse_no_data() {
        let las_content = "~VERSION INFORMATION
 VERS.  2.0
~CURVE INFORMATION
 DEPT.M : DEPTH
~A
";
        let temp_dir = create_test_las(las_content);
        let las_path = temp_dir.path().join("test.las");

        let result = LasParser::read(&las_path);
        assert!(result.is_err());
    }
}
