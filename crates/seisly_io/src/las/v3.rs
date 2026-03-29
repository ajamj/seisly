//! LAS 3.0 file parser
//!
//! LAS 3.0 enhancements:
//! - JSON-like metadata sections
//! - Enhanced curve definitions
//! - Better encoding support (UTF-8)

use crate::las::parser::LasError;
use sf_core::domain::well::{Well, WellLog};
use std::io::{BufRead, BufReader};
use std::path::Path;

/// LAS 3.0 file reader
pub struct LasV3Reader {
    path: std::path::PathBuf,
}

impl LasV3Reader {
    /// Open a LAS 3.0 file
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LasError> {
        Ok(Self {
            path: path.as_ref().to_path_buf(),
        })
    }

    /// Parse the LAS file and return Well structure
    pub fn parse(&self) -> Result<Well, LasError> {
        let file = std::fs::File::open(&self.path)
            .map_err(|_| LasError::FileNotFound(self.path.display().to_string()))?;
        let reader = BufReader::new(file);

        let mut version_section = Vec::new();
        let mut well_section = Vec::new();
        let mut curve_section = Vec::new();
        let mut data_section = Vec::new();
        let mut current_section = String::new();
        let mut null_value: f32 = -999.25;

        for line in reader.lines() {
            let line = line.map_err(|e| LasError::ParseError(e.to_string()))?;
            let line = line.trim();

            if line.starts_with('~') {
                current_section = line.to_uppercase();
                continue;
            }

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if current_section.contains("VERSION") {
                version_section.push(line.to_string());
                // Check for NULL value in version section
                if line.starts_with(" NULL.") {
                    null_value = Self::extract_numeric_value(line).unwrap_or(-999.25);
                }
            } else if current_section.contains("WELL") {
                well_section.push(line.to_string());
            } else if current_section.contains("CURVE") {
                if let Some(curve_def) = Self::parse_curve_definition(line) {
                    curve_section.push(curve_def);
                }
            } else if current_section.contains("ASCII") || current_section.contains("DATA") {
                let values: Vec<f32> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect();
                if !values.is_empty() {
                    data_section.push(values);
                }
            }
        }

        // Build Well structure
        let well_name = Self::extract_well_name(&well_section);

        // Create depth vector from first column of data
        let mut depths: Vec<f32> = Vec::new();
        for row in &data_section {
            if let Some(&depth) = row.first() {
                depths.push(depth);
            }
        }

        // Create logs for each curve
        let mut logs: Vec<WellLog> = Vec::new();
        for (i, curve_def) in curve_section.iter().enumerate() {
            // Skip depth column (index 0) for data extraction
            let data: Vec<f32> = data_section
                .iter()
                .filter_map(|row| row.get(i + 1).copied())
                .filter(|&val| val != null_value)
                .collect();

            // Create a copy of depths for this log
            let log_depths = depths.clone();

            let mut log = WellLog::new(
                curve_def.mnemonic.clone(),
                curve_def.unit.clone(),
                data,
                log_depths,
            );
            log.description = curve_def.description.clone();
            logs.push(log);
        }

        // Create Well model
        let mut well = Well::new(
            well_name.clone(),
            well_name.clone(),
            0.0, // X coordinate (not in LAS, set to 0)
            0.0, // Y coordinate (not in LAS, set to 0)
            0.0, // Elevation (not in LAS, set to 0)
        );
        well.logs = logs;

        Ok(well)
    }

    /// Extract well name from well section
    fn extract_well_name(well_section: &[String]) -> String {
        for line in well_section {
            let trimmed = line.trim();
            if trimmed.starts_with("WELL.") {
                if let Some(value) = Self::extract_value(trimmed) {
                    return value;
                }
            }
        }
        "Unknown".to_string()
    }

    /// Parse a curve definition line
    fn parse_curve_definition(line: &str) -> Option<CurveDef> {
        if line.starts_with('~') || line.is_empty() {
            return None;
        }

        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() < 2 {
            return None;
        }

        let mnemonic_unit = parts[0].trim();
        let description = parts[1].trim();

        // Parse mnemonic and unit (e.g., "DEPT.M" or "GR.GAPI")
        if let Some(dot_pos) = mnemonic_unit.find('.') {
            let mnemonic = mnemonic_unit[..dot_pos].trim().to_string();
            let unit = mnemonic_unit[dot_pos + 1..]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();

            // Skip if mnemonic looks like a section header or is too long
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
        // Format: " KEY. VALUE : Description" or " KEY. VALUE : Description"
        let line = line.trim_start();
        if let Some(dot_pos) = line.find('.') {
            let after_dot = line[dot_pos + 1..].trim();
            let value_str = after_dot.split_whitespace().next()?;
            // Take only the numeric part (before any colon)
            let numeric_part = value_str.split(':').next()?;
            numeric_part.parse().ok()
        } else {
            None
        }
    }

    /// Extract string value from LAS line
    fn extract_value(line: &str) -> Option<String> {
        // Format: " KEY. VALUE : Description"
        let line = line.trim_start();
        if let Some(dot_pos) = line.find('.') {
            let after_dot = line[dot_pos + 1..].trim();
            // Get the value before the colon (description separator)
            let value = after_dot.split(':').next()?.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
        None
    }
}

/// Curve definition from LAS header
#[derive(Debug, Clone)]
pub struct CurveDef {
    pub mnemonic: String,
    pub unit: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_definition_parser() {
        let curve = LasV3Reader::parse_curve_definition("GR.GAPI 00 000 : Gamma Ray").unwrap();
        assert_eq!(curve.mnemonic, "GR");
        assert_eq!(curve.unit, "GAPI");
        assert_eq!(curve.description, "Gamma Ray");
    }

    #[test]
    fn test_curve_definition_parser_dept() {
        let curve = LasV3Reader::parse_curve_definition("DEPT.M : Depth").unwrap();
        assert_eq!(curve.mnemonic, "DEPT");
        assert_eq!(curve.unit, "M");
        assert_eq!(curve.description, "Depth");
    }

    #[test]
    fn test_extract_value() {
        let value = LasV3Reader::extract_value(" WELL. TEST-WELL : Well Name");
        assert_eq!(value, Some("TEST-WELL".to_string()));
    }

    #[test]
    fn test_extract_numeric_value() {
        let value = LasV3Reader::extract_numeric_value(" NULL. -999.25 : Null value");
        assert_eq!(value, Some(-999.25));
    }
}
