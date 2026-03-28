use sf_core::domain::surface::Mesh;

pub struct ThrowPoint {
    pub position: [f32; 3],
    pub throw_value: f32,
}

/// Calculate vertical throw distribution along an intersection line between two mesh components.
pub fn calculate_throw_distribution(
    up_mesh: &Mesh,
    down_mesh: &Mesh,
    intersection_line: &[[f32; 3]],
) -> Vec<ThrowPoint> {
    let mut distribution = Vec::with_capacity(intersection_line.len());

    for &point in intersection_line {
        // Find nearest Z on up_mesh and down_mesh at this (X, Y)
        // For now, use a simplified nearest-vertex approach.
        // TODO: Implement proper ray-casting or barycentric interpolation for exact Z.
        let z_up = find_nearest_z(up_mesh, point[0], point[1]);
        let z_down = find_nearest_z(down_mesh, point[0], point[1]);

        if let (Some(zu), Some(zd)) = (z_up, z_down) {
            distribution.push(ThrowPoint {
                position: point,
                throw_value: (zu - zd).abs(),
            });
        }
    }

    distribution
}

fn find_nearest_z(mesh: &Mesh, x: f32, y: f32) -> Option<f32> {
    let mut min_dist_sq = f32::MAX;
    let mut nearest_z = None;

    for v in &mesh.vertices {
        let dx = v[0] - x;
        let dy = v[1] - y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq < min_dist_sq && dist_sq < 100.0 {
            // 10 unit radius
            min_dist_sq = dist_sq;
            nearest_z = Some(v[2]);
        }
    }

    nearest_z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_throw() {
        let up_mesh = Mesh::new(
            vec![[0.0, 0.0, 100.0], [10.0, 0.0, 100.0], [0.0, 10.0, 100.0]],
            vec![0, 1, 2],
        );
        let down_mesh = Mesh::new(
            vec![[0.0, 0.0, 80.0], [10.0, 0.0, 80.0], [0.0, 10.0, 80.0]],
            vec![0, 1, 2],
        );
        let line = vec![[5.0, 5.0, 90.0]];

        let dist = calculate_throw_distribution(&up_mesh, &down_mesh, &line);
        assert_eq!(dist.len(), 1);
        assert!((dist[0].throw_value - 20.0).abs() < 1e-3);
    }
}
