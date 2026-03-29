//! Well Data Models
//!
//! Provides data structures for well information including well tops, logs, and trajectory.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Well information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Well {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub location: WellLocation,
    pub datum: WellDatum,
    pub logs: Vec<WellLog>,
    pub tops: Vec<WellTop>,
    pub is_visible: bool,
}

/// Well surface location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellLocation {
    pub x: f64,      // Easting (UTM or local coordinates)
    pub y: f64,      // Northing
    pub crs: String, // Coordinate Reference System
}

/// Well datum information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellDatum {
    pub name: String,   // e.g., "KB", "GL", "MSL"
    pub elevation: f32, // Elevation relative to sea level (meters)
}

/// Well log curve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellLog {
    pub id: Uuid,
    pub mnemonic: String, // e.g., "GR", "DT", "RHOB"
    pub description: String,
    pub units: String,
    pub data: Vec<f32>,   // Log values
    pub depths: Vec<f32>, // Depth values (in meters)
    pub min_depth: f32,
    pub max_depth: f32,
}

/// Well top (formation marker)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellTop {
    pub id: Uuid,
    pub name: String,    // Formation name
    pub depth: f32,      // Depth in meters
    pub type_: String,   // "TOP", "BASE", "ZONE"
    pub color: [f32; 4], // RGBA color for visualization
}

impl Well {
    pub fn new(name: String, symbol: String, x: f64, y: f64, elevation: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            symbol,
            location: WellLocation {
                x,
                y,
                crs: "UTM".to_string(),
            },
            datum: WellDatum {
                name: "KB".to_string(),
                elevation,
            },
            logs: Vec::new(),
            tops: Vec::new(),
            is_visible: true,
        }
    }

    pub fn add_log(&mut self, mnemonic: String, units: String, data: Vec<f32>, depths: Vec<f32>) {
        let min_depth = *depths
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let max_depth = *depths
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);

        self.logs.push(WellLog {
            id: Uuid::new_v4(),
            mnemonic,
            description: String::new(),
            units,
            data,
            depths,
            min_depth,
            max_depth,
        });
    }

    pub fn add_top(&mut self, name: String, depth: f32, type_: String, color: [f32; 4]) {
        self.tops.push(WellTop {
            id: Uuid::new_v4(),
            name,
            depth,
            type_,
            color,
        });
    }
}

impl WellLog {
    pub fn new(mnemonic: String, units: String, data: Vec<f32>, depths: Vec<f32>) -> Self {
        let min_depth = *depths
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let max_depth = *depths
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);

        Self {
            id: Uuid::new_v4(),
            mnemonic,
            description: String::new(),
            units,
            data,
            depths,
            min_depth,
            max_depth,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_well_creation() {
        let well = Well::new(
            "Well-1".to_string(),
            "W1".to_string(),
            500000.0,
            1000000.0,
            100.0,
        );
        assert_eq!(well.name, "Well-1");
        assert_eq!(well.symbol, "W1");
        assert_eq!(well.datum.elevation, 100.0);
        assert!(well.is_visible);
    }

    #[test]
    fn test_well_log_addition() {
        let mut well = Well::new(
            "Well-1".to_string(),
            "W1".to_string(),
            500000.0,
            1000000.0,
            100.0,
        );
        let depths = vec![0.0, 100.0, 200.0, 300.0];
        let gr_values = vec![50.0, 60.0, 70.0, 80.0];

        well.add_log(
            "GR".to_string(),
            "GAPI".to_string(),
            gr_values.clone(),
            depths.clone(),
        );

        assert_eq!(well.logs.len(), 1);
        assert_eq!(well.logs[0].mnemonic, "GR");
        assert_eq!(well.logs[0].data, gr_values);
        assert_eq!(well.logs[0].min_depth, 0.0);
        assert_eq!(well.logs[0].max_depth, 300.0);
    }

    #[test]
    fn test_well_top_addition() {
        let mut well = Well::new(
            "Well-1".to_string(),
            "W1".to_string(),
            500000.0,
            1000000.0,
            100.0,
        );
        well.add_top(
            "Formation A".to_string(),
            1500.0,
            "TOP".to_string(),
            [1.0, 0.0, 0.0, 1.0],
        );

        assert_eq!(well.tops.len(), 1);
        assert_eq!(well.tops[0].name, "Formation A");
        assert_eq!(well.tops[0].depth, 1500.0);
    }
}
