//! LAS (Log ASCII Standard) file parser

use sf_core::domain::log::{Log, Curve, DepthMnemonic};
use sf_core::EntityId;
use thiserror::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Error, Debug)]
pub enum LasError {
    #[error("Failed to read LAS file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid LAS format: {0}")]
    ParseError(String),
    #[error("No data section found")]
    NoDataSection,
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
    pub fn parse(path: &Path) -> Result<Log, LasError> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut curves: Vec<Curve> = Vec::new();
        let mut in_data = false;
        let mut depth_unit = "M".to_string();
        let mut null_value = -999.25f32;
        let mut curve_defs: Vec<CurveDef> = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            
            if line.is_empty() {
                continue;
            }
            
            // Check for NULL value in ~PARAMETER section
            if line.starts_with("NULL.") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    null_value = val.parse().unwrap_or(-999.25);
                }
            }
            
            // Check for depth unit in ~WELL section
            if line.starts_with("STRT.") || line.starts_with("STOP.") {
                if let Some(unit) = line.split('.').nth(1).and_then(|s| s.split_whitespace().next()) {
                    depth_unit = unit.to_string();
                }
            }
            
            // Parse curve definitions in ~CURVE section
            if line.starts_with('~') {
                in_data = line.to_uppercase().starts_with("~A");
                continue;
            }
            
            if in_data {
                // Parse data line
                let values: Vec<f32> = line
                    .split_whitespace()
                    .filter_map(|v| v.parse().ok())
                    .collect();
                
                if values.is_empty() {
                    continue;
                }
                
                // First value is depth, rest are curve values
                for (i, &val) in values.iter().enumerate().skip(1) {
                    if i > curves.len() {
                        // Use curve definition if available, otherwise generate name
                        let (mnemonic, unit) = if i - 1 < curve_defs.len() {
                            (curve_defs[i - 1].mnemonic.clone(), curve_defs[i - 1].unit.clone())
                        } else {
                            (format!("CURVE_{}", i), "UNKNOWN".to_string())
                        };
                        
                        curves.push(Curve {
                            mnemonic,
                            unit,
                            values: vec![val],
                            null_value,
                        });
                    } else {
                        curves[i - 1].values.push(val);
                    }
                }
            } else if line.starts_with(|c: char| c.is_whitespace() || c.is_alphabetic()) && !line.starts_with('~') {
                // Try to parse curve definition (e.g., " DEPT.M                      : 1  DEPTH")
                if let Some(curve_def) = Self::parse_curve_def(line) {
                    curve_defs.push(curve_def);
                }
            }
        }
        
        if curves.is_empty() {
            return Err(LasError::NoDataSection);
        }
        
        Ok(Log {
            id: EntityId::new_v4(),
            well_id: EntityId::new_v4(),
            depth_mnemonic: DepthMnemonic::MD,
            depth_unit,
            curves,
        })
    }
    
    /// Parse a curve definition line
    fn parse_curve_def(line: &str) -> Option<CurveDef> {
        // Format: " MNEM.UNIT          : VALUE DESCRIPTION"
        let line = line.trim();
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
            let unit = mnemonic_unit[dot_pos + 1..].trim().split_whitespace().next().unwrap_or("").to_string();
            
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::io::Write;

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
        
        let log = LasParser::parse(&las_path).unwrap();
        assert!(!log.curves.is_empty());
        assert!(log.curves[0].values.len() >= 2);
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
        
        let result = LasParser::parse(&las_path);
        assert!(result.is_err());
    }
}
