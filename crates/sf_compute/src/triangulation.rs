//! Delaunay triangulation for surface building

use sf_core::domain::surface::Mesh;
use spade::{DelaunayTriangulation, Point2, HasPosition, Triangulation};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriangulationError {
    #[error("Not enough points for triangulation (minimum 3)")]
    NotEnoughPoints,
    #[error("Triangulation failed: {0}")]
    SpadeError(String),
}

struct IndexedPoint {
    point: Point2<f64>,
    index: usize,
}

impl HasPosition for IndexedPoint {
    type Scalar = f64;
    fn position(&self) -> Point2<f64> {
        self.point
    }
}

/// Triangulate 3D points using Delaunay triangulation
/// 
/// Projects points to 2D (XY plane) for triangulation,
/// then reconstructs 3D mesh with original Z values.
pub fn triangulate_points(points: &[[f32; 3]]) -> Result<Mesh, TriangulationError> {
    if points.len() < 3 {
        return Err(TriangulationError::NotEnoughPoints);
    }
    
    // Convert to 2D points for Delaunay (using x, y) with original indices
    let points_2d: Vec<IndexedPoint> = points.iter()
        .enumerate()
        .map(|(i, p)| IndexedPoint {
            point: Point2::new(p[0] as f64, p[1] as f64),
            index: i,
        })
        .collect();
    
    let triangulation = DelaunayTriangulation::<IndexedPoint>::bulk_load(points_2d)
        .map_err(|e| TriangulationError::SpadeError(e.to_string()))?;
    
    let vertices = points.to_vec();
    let mut indices = Vec::new();
    
    // Extract triangle indices from triangulation
    for face in triangulation.inner_faces() {
        let face_vertices = face.vertices();
        indices.push(face_vertices[0].data().index as u32);
        indices.push(face_vertices[1].data().index as u32);
        indices.push(face_vertices[2].data().index as u32);
    }
    
    let mut mesh = Mesh::new(vertices, indices);
    mesh.compute_normals();
    
    Ok(mesh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulation_square() {
        let points = vec![
            [0.0, 0.0, 100.0],
            [100.0, 0.0, 105.0],
            [0.0, 100.0, 103.0],
            [100.0, 100.0, 108.0],
        ];
        
        let mesh = triangulate_points(&points).unwrap();
        assert_eq!(mesh.vertices.len(), 4);
        assert!(!mesh.indices.is_empty());
        // Should have 2 triangles (6 indices) for a quad
        assert!(mesh.indices.len() >= 6);
    }

    #[test]
    fn test_triangulation_not_enough_points() {
        let points = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ];
        
        let result = triangulate_points(&points);
        assert!(result.is_err());
    }
}
