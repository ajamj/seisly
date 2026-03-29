//! Coordinate Reference System definitions

use serde::{Deserialize, Serialize};

/// Coordinate Reference System definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crs {
    /// Authority name (e.g., "EPSG")
    pub authority: Option<String>,
    /// Authority code (e.g., "32648")
    pub code: Option<String>,
    /// Full definition string (WKT2 or PROJ string)
    pub definition: String,
}

impl Crs {
    /// WGS84 coordinate reference system (EPSG:4326)
    pub fn wgs84() -> Self {
        Self {
            authority: Some("EPSG".to_string()),
            code: Some("4326".to_string()),
            definition: "EPSG:4326".to_string(),
        }
    }

    /// Create a CRS from EPSG code
    pub fn from_epsg(code: u32) -> Self {
        Self {
            authority: Some("EPSG".to_string()),
            code: Some(code.to_string()),
            definition: format!("EPSG:{}", code),
        }
    }

    /// Get the EPSG code if available
    pub fn epsg_code(&self) -> Option<u32> {
        if self.authority.as_deref() == Some("EPSG") {
            self.code.as_ref().and_then(|c| c.parse().ok())
        } else {
            None
        }
    }
}

impl Default for Crs {
    fn default() -> Self {
        Self::from_epsg(4326) // WGS84
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crs_from_epsg() {
        let crs = Crs::from_epsg(32648);
        assert_eq!(crs.authority, Some("EPSG".to_string()));
        assert_eq!(crs.code, Some("32648".to_string()));
        assert_eq!(crs.definition, "EPSG:32648");
        assert_eq!(crs.epsg_code(), Some(32648));
    }

    #[test]
    fn test_crs_default() {
        let crs = Crs::default();
        assert_eq!(crs.epsg_code(), Some(4326));
    }
}
