//! Project management for StrataForge

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Project data structure for serialization
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub name: String,
    pub version: String,
    pub created_at: String,
    pub modified_at: String,
    pub interpretation: InterpretationSnapshot,
    pub wells: WellStateSnapshot,
}

/// Snapshot of interpretation state
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpretationSnapshot {
    pub horizons: Vec<HorizonSnapshot>,
    pub faults: Vec<FaultSnapshot>,
    pub active_horizon_id: Option<uuid::Uuid>,
    pub active_fault_id: Option<uuid::Uuid>,
}

/// Snapshot of well state
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellStateSnapshot {
    pub wells: Vec<WellSnapshot>,
    pub active_well_id: Option<uuid::Uuid>,
}

// Simplified snapshots for now - will be expanded
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizonSnapshot {
    pub id: uuid::Uuid,
    pub name: String,
    pub color: [f32; 4],
    pub picks_count: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultSnapshot {
    pub id: uuid::Uuid,
    pub name: String,
    pub color: [f32; 4],
    pub sticks_count: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellSnapshot {
    pub id: uuid::Uuid,
    pub name: String,
    pub logs_count: usize,
}

pub struct ProjectManager;

impl ProjectManager {
    /// Create a new project
    pub fn create_new(name: &str) -> ProjectData {
        let now = "2026-03-28T00:00:00Z"; // Simplified timestamp
        ProjectData {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            created_at: now.to_string(),
            modified_at: now.to_string(),
            interpretation: InterpretationSnapshot {
                horizons: Vec::new(),
                faults: Vec::new(),
                active_horizon_id: None,
                active_fault_id: None,
            },
            wells: WellStateSnapshot {
                wells: Vec::new(),
                active_well_id: None,
            },
        }
    }

    /// Save project to file
    pub fn save(project: &ProjectData, path: &Path) -> Result<(), String> {
        let json =
            serde_json::to_string_pretty(project).map_err(|e| format!("JSON error: {}", e))?;
        std::fs::write(path, json).map_err(|e| format!("IO error: {}", e))?;
        Ok(())
    }

    /// Load project from file
    pub fn load(path: &Path) -> Result<ProjectData, String> {
        if !path.exists() {
            return Err(format!("Project not found: {}", path.display()));
        }
        let content = std::fs::read_to_string(path).map_err(|e| format!("IO error: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("JSON error: {}", e))
    }

    /// Get project file extension
    #[allow(dead_code)]
    pub const EXTENSION: &'static str = "sfp"; // StrataForge Project
}

// Conversion methods (to be implemented as needed)
impl ProjectData {
    #[allow(dead_code)]
    pub fn from_state(
        name: &str,
        interpretation: &crate::interpretation::InterpretationState,
        wells: &crate::interpretation::WellState,
    ) -> Self {
        let now = "2026-03-28T00:00:00Z";
        let mut project = ProjectManager::create_new(name);
        project.modified_at = now.to_string();

        // Snapshot interpretation
        project.interpretation = InterpretationSnapshot {
            horizons: interpretation
                .horizons
                .iter()
                .map(|h| HorizonSnapshot {
                    id: h.id,
                    name: h.name.clone(),
                    color: h.color,
                    picks_count: h.picks.len(),
                })
                .collect(),
            faults: interpretation
                .faults
                .iter()
                .map(|f| FaultSnapshot {
                    id: f.id,
                    name: f.name.clone(),
                    color: f.color,
                    sticks_count: f.sticks.len(),
                })
                .collect(),
            active_horizon_id: interpretation.active_horizon_id,
            active_fault_id: interpretation.active_fault_id,
        };

        // Snapshot wells
        project.wells = WellStateSnapshot {
            wells: wells
                .wells
                .iter()
                .map(|w| WellSnapshot {
                    id: w.id,
                    name: w.name.clone(),
                    logs_count: w.logs.len(),
                })
                .collect(),
            active_well_id: wells.active_well_id,
        };

        project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_and_save_project() {
        let project = ProjectManager::create_new("Test Project");
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.sfp");

        let result = ProjectManager::save(&project, &path);
        assert!(result.is_ok());
        assert!(path.exists());
    }

    #[test]
    fn test_load_project() {
        let project = ProjectManager::create_new("Test Project");
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.sfp");

        ProjectManager::save(&project, &path).unwrap();

        let loaded = ProjectManager::load(&path).unwrap();
        assert_eq!(loaded.name, "Test Project");
    }
}
