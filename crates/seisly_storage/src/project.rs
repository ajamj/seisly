//! Project manifest and format handling

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("Failed to read project file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse project YAML: {0}")]
    ParseError(#[from] serde_yaml::Error),
    #[error("Invalid project path: {0}")]
    InvalidPath(String),
    #[error("Project not found: {0}")]
    NotFound(String),
}

/// Project manifest (project.yaml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub name: String,
    pub default_crs: String,
    pub created_at: String,
    pub version: String,
    #[serde(default)]
    pub datasets: Vec<String>,
    #[serde(default)]
    pub seismic_volumes: Vec<SeismicVolumeEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeismicVolumeEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub is_visible: bool,
    pub channel_assignment: u8, // 0: None, 1: Red, 2: Green, 3: Blue
}

impl ProjectManifest {
    pub fn new(name: String, default_crs: String) -> Self {
        Self {
            name,
            default_crs,
            created_at: chrono::Utc::now().to_rfc3339(),
            version: "0.1.0".to_string(),
            datasets: vec![],
            seismic_volumes: vec![],
        }
    }

    pub fn load(project_path: &Path) -> Result<Self, ProjectError> {
        let manifest_path = project_path.join("project.yaml");
        if !manifest_path.exists() {
            return Err(ProjectError::NotFound(manifest_path.display().to_string()));
        }
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: ProjectManifest = serde_yaml::from_str(&content)?;
        Ok(manifest)
    }

    pub fn save(&self, project_path: &Path) -> Result<(), ProjectError> {
        let manifest_path = project_path.join("project.yaml");
        let content = serde_yaml::to_string(self)?;
        std::fs::write(&manifest_path, content)?;
        Ok(())
    }
}

/// Represents an open StrataForge project
pub struct Project {
    pub path: PathBuf,
    pub manifest: ProjectManifest,
}

impl Project {
    /// Create a new project at the specified path
    pub fn create(path: PathBuf, name: String, default_crs: String) -> Result<Self, ProjectError> {
        // Create directory structure
        std::fs::create_dir_all(&path)?;
        std::fs::create_dir_all(path.join("blobs"))?;
        std::fs::create_dir_all(path.join("cache"))?;
        std::fs::create_dir_all(path.join("cache").join("tiles"))?;
        std::fs::create_dir_all(path.join("cache").join("decimated"))?;
        std::fs::create_dir_all(path.join("workflows"))?;
        std::fs::create_dir_all(path.join("workflows").join("runs"))?;
        std::fs::create_dir_all(path.join("logs"))?;

        let manifest = ProjectManifest::new(name, default_crs);
        manifest.save(&path)?;

        Ok(Self { path, manifest })
    }

    /// Open an existing project
    pub fn open(path: PathBuf) -> Result<Self, ProjectError> {
        if !path.exists() {
            return Err(ProjectError::NotFound(path.display().to_string()));
        }
        let manifest = ProjectManifest::load(&path)?;
        Ok(Self { path, manifest })
    }

    /// Get the metadata database path
    pub fn metadata_path(&self) -> PathBuf {
        self.path.join("metadata.sqlite")
    }

    /// Get the blob store path
    pub fn blobs_path(&self) -> PathBuf {
        self.path.join("blobs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_project_creation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("TestProject.sf");

        let project = Project::create(
            project_path.clone(),
            "Test Project".to_string(),
            "EPSG:32648".to_string(),
        )
        .unwrap();

        assert_eq!(project.manifest.name, "Test Project");
        assert_eq!(project.manifest.default_crs, "EPSG:32648");
        assert!(project_path.join("project.yaml").exists());
        assert!(project_path.join("blobs").exists());
        assert!(project_path.join("cache").exists());
    }

    #[test]
    fn test_project_open() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("TestProject.sf");

        Project::create(
            project_path.clone(),
            "Test".to_string(),
            "EPSG:4326".to_string(),
        )
        .unwrap();

        let project = Project::open(project_path.clone()).unwrap();
        assert_eq!(project.manifest.name, "Test");
    }

    #[test]
    fn test_project_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("NonExistent.sf");

        let result = Project::open(project_path);
        assert!(result.is_err());
    }
}
