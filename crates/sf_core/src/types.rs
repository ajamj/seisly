//! Common types used across the domain model

use crate::Crs;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for all entities
pub type EntityId = Uuid;

/// Provenance tracking for reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Source dataset IDs
    pub source_ids: Vec<EntityId>,
    /// Algorithm name
    pub algorithm: String,
    /// Algorithm version
    pub algorithm_version: String,
    /// Parameters as JSON
    pub parameters: serde_json::Value,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Common metadata for all datasets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub id: EntityId,
    pub name: String,
    pub crs: Crs,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub provenance: Option<Provenance>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provenance_creation() {
        let provenance = Provenance {
            source_ids: vec![Uuid::new_v4()],
            algorithm: "test_algo".to_string(),
            algorithm_version: "1.0.0".to_string(),
            parameters: serde_json::json!({"param": "value"}),
            created_at: Utc::now(),
        };

        assert_eq!(provenance.source_ids.len(), 1);
        assert_eq!(provenance.algorithm, "test_algo");
    }
}
