//! Formation Top domain model
//!
//! Represents a stratigraphic horizon picked on a well log or seismic.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a formation top
pub type FormationTopId = Uuid;

/// A formation top (marker) picked on a well
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormationTop {
    /// Unique identifier
    pub id: FormationTopId,
    /// Reference to the well this top belongs to
    pub well_id: Uuid,
    /// Name of the formation top (e.g., "Top Reservoir", "Base Seal")
    pub name: String,
    /// Measured depth in meters
    pub depth_md: f64,
    /// Optional formation name
    pub formation: Option<String>,
    /// Optional comments
    pub comments: Option<String>,
}

impl FormationTop {
    /// Create a new formation top
    pub fn new(
        well_id: Uuid,
        name: String,
        depth_md: f64,
        formation: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            well_id,
            name,
            depth_md,
            formation,
            comments: None,
        }
    }

    /// Set optional comments
    pub fn with_comments(mut self, comments: String) -> Self {
        self.comments = Some(comments);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formation_top_creation() {
        let well_id = Uuid::new_v4();
        let top = FormationTop::new(
            well_id,
            "Top Reservoir".to_string(),
            2500.0,
            Some("Formation A".to_string()),
        );

        assert_eq!(top.well_id, well_id);
        assert_eq!(top.name, "Top Reservoir");
        assert_eq!(top.depth_md, 2500.0);
        assert_eq!(top.formation, Some("Formation A".to_string()));
    }

    #[test]
    fn test_formation_top_serialization() {
        let well_id = Uuid::new_v4();
        let top = FormationTop::new(
            well_id,
            "Top Seal".to_string(),
            1800.5,
            None,
        );

        let json = serde_json::to_string(&top).unwrap();
        assert!(json.contains("Top Seal"));
        assert!(json.contains("1800.5"));
    }

    #[test]
    fn test_with_comments_builder() {
        let well_id = Uuid::new_v4();
        let top = FormationTop::new(
            well_id,
            "Top Reservoir".to_string(),
            2500.0,
            None,
        )
        .with_comments("Test comment".to_string());

        assert_eq!(top.comments, Some("Test comment".to_string()));
    }

    #[test]
    fn test_formation_top_equality() {
        let well_id = Uuid::new_v4();
        let top1 = FormationTop::new(
            well_id,
            "Top Reservoir".to_string(),
            2500.0,
            Some("Formation A".to_string()),
        );

        let top2 = FormationTop::new(
            well_id,
            "Top Reservoir".to_string(),
            2500.0,
            Some("Formation A".to_string()),
        );

        // IDs will be different (UUIDs are unique)
        assert_ne!(top1.id, top2.id);
        
        // But we can manually compare other fields
        assert_eq!(top1.well_id, top2.well_id);
        assert_eq!(top1.name, top2.name);
        assert_eq!(top1.depth_md, top2.depth_md);
        assert_eq!(top1.formation, top2.formation);
    }
}
