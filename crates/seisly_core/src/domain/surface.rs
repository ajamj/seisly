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

    /// Compute vertex normals by averaging face normals with area weighting.
    /// 
    /// Area-weighted normals prevent small triangles from disproportionately affecting
    /// the surface appearance, resulting in smoother shading for horizons with varying
    /// triangle sizes.
    pub fn compute_normals(&mut self) {
        let mut normals = vec![[0.0, 0.0, 0.0]; self.vertices.len()];

        // Iterate through each triangle
        for chunk in self.indices.chunks_exact(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let v0 = self.vertices[i0];
            let v1 = self.vertices[i1];
            let v2 = self.vertices[i2];

            // Edge vectors
            let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            // Cross product (e1 x e2) gives area-weighted face normal
            // The magnitude of the cross product is 2 * triangle area
            let face_normal = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];

            // Area-weighted accumulation: larger triangles contribute more to vertex normals
            // The cross product magnitude is already proportional to triangle area
            for &idx in chunk {
                let n = &mut normals[idx as usize];
                n[0] += face_normal[0];
                n[1] += face_normal[1];
                n[2] += face_normal[2];
            }
        }

        // Normalize each vertex normal
        for n in &mut normals {
            let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if length > 1e-6 {
                n[0] /= length;
                n[1] /= length;
                n[2] /= length;
            } else {
                // Fallback for zero-length normal (degenerate case)
                *n = [0.0, 1.0, 0.0];
            }
        }

        self.normals = Some(normals);
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

    #[test]
    fn test_compute_normals() {
        // Triangle in XY plane (pointing up in Z)
        let vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let indices = vec![0, 1, 2];
        let mut mesh = Mesh::new(vertices, indices);
        
        mesh.compute_normals();
        
        let normals = mesh.normals.expect("Normals should be computed");
        assert_eq!(normals.len(), 3);
        
        // Each vertex normal should be [0, 0, 1]
        for n in normals {
            assert!(n[0].abs() < 1e-6);
            assert!(n[1].abs() < 1e-6);
            assert!((n[2] - 1.0).abs() < 1e-6);
        }
    }
}
