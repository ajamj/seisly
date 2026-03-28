//! Well domain entity

use crate::types::{DatasetMetadata, EntityId};
use crate::Crs;

/// Well head information
#[derive(Debug, Clone)]
pub struct Well {
    pub metadata: DatasetMetadata,
    /// X coordinate of well head (in project CRS)
    pub head_x: f64,
    /// Y coordinate of well head (in project CRS)
    pub head_y: f64,
    /// Kelly bushing elevation (optional, in meters)
    pub kb_elevation: Option<f64>,
}

impl Well {
    pub fn new(name: String, crs: Crs, head_x: f64, head_y: f64) -> Self {
        Self {
            metadata: DatasetMetadata {
                id: EntityId::new_v4(),
                name,
                crs,
                created_at: chrono::Utc::now(),
                tags: vec![],
                provenance: None,
            },
            head_x,
            head_y,
            kb_elevation: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_well_creation() {
        let crs = Crs::from_epsg(32648);
        let well = Well::new("Well-1".to_string(), crs, 500000.0, 1000000.0);
        assert_eq!(well.metadata.name, "Well-1");
        assert_eq!(well.head_x, 500000.0);
        assert_eq!(well.head_y, 1000000.0);
        assert!(well.kb_elevation.is_none());
    }
}
