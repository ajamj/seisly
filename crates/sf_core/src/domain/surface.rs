//! Surface domain entity

use crate::types::{DatasetMetadata, EntityId};
use crate::Crs;

/// Reference to a blob in the blob store
#[derive(Debug, Clone)]
pub struct BlobRef {
    pub hash: String,
    pub size_bytes: u64,
}

/// Surface mesh representation
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Option<Vec<[f32; 3]>>,
}

impl Mesh {
    pub fn new(vertices: Vec<[f32; 3]>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            normals: None,
        }
    }

    /// Compute face normals (placeholder implementation)
    pub fn compute_normals(&mut self) {
        // TODO: Implement proper normal calculation
        self.normals = Some(vec![[0.0, 1.0, 0.0]; self.vertices.len()]);
    }
}

/// Geological surface
#[derive(Debug, Clone)]
pub struct Surface {
    pub metadata: DatasetMetadata,
    pub mesh_ref: BlobRef,
}

impl Surface {
    pub fn new(name: String, crs: Crs, mesh_ref: BlobRef) -> Self {
        Self {
            metadata: DatasetMetadata {
                id: EntityId::new_v4(),
                name,
                crs,
                created_at: chrono::Utc::now(),
                tags: vec![],
                provenance: None,
            },
            mesh_ref,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        let vertices = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
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
}
