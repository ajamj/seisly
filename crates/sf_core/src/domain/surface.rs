//! Surface domain entity

use crate::types::{DatasetMetadata, EntityId};
use crate::Crs;
use serde::{Deserialize, Serialize};

/// Reference to a blob in the blob store (deprecated for in-memory surface mesh)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobRef {
    pub hash: String,
    pub size_bytes: u64,
}

/// Surface mesh representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub colors: Option<Vec<[f32; 3]>>,
}

impl Mesh {
    pub fn new(vertices: Vec<[f32; 3]>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            normals: None,
            colors: None,
        }
    }

    /// Compute face normals (placeholder implementation)
    pub fn compute_normals(&mut self) {
        // TODO: Implement proper normal calculation
        self.normals = Some(vec![[0.0, 1.0, 0.0]; self.vertices.len()]);
    }
}

/// Geological surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surface {
    pub metadata: DatasetMetadata,
    pub meshes: Vec<Mesh>,
    pub intersection_lines: Vec<Vec<[f32; 3]>>,
}

impl Surface {
    pub fn new(name: String, crs: Crs, meshes: Vec<Mesh>) -> Self {
        Self {
            metadata: DatasetMetadata {
                id: EntityId::new_v4(),
                name,
                crs,
                created_at: chrono::Utc::now(),
                tags: vec![],
                provenance: None,
            },
            meshes,
            intersection_lines: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        let vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let indices = vec![0, 1, 2];
        let mesh = Mesh::new(vertices, indices);
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.indices.len(), 3);
        assert!(mesh.normals.is_none());
    }

    #[test]
    fn test_blob_ref_creation() {
        let blob_ref = BlobRef {
            hash: "abc123".to_string(),
            size_bytes: 1024,
        };
        assert_eq!(blob_ref.hash, "abc123");
        assert_eq!(blob_ref.size_bytes, 1024);
    }

    #[test]
    fn test_surface_creation() {
        let vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let indices = vec![0, 1, 2];
        let mesh = Mesh::new(vertices, indices);
        let crs = crate::Crs::wgs84();
        let surface = Surface::new("Top Reservoir".to_string(), crs, vec![mesh]);

        assert_eq!(surface.metadata.name, "Top Reservoir");
        assert_eq!(surface.meshes.len(), 1);
        assert_eq!(surface.intersection_lines.len(), 0);
    }
}
