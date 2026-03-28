//! Radial Basis Function (RBF) interpolation for surface modeling

use nalgebra::{DMatrix, DVector};
use sf_core::domain::surface::Mesh;

/// Type of Radial Basis Function to use
#[derive(Debug, Clone, Copy)]
pub enum RbfType {
    /// Thin Plate Spline: r^2 * ln(r)
    ThinPlateSpline,
    /// Multiquadric: sqrt(1 + (epsilon * r)^2)
    Multiquadric { epsilon: f32 },
    /// Gaussian: exp(-(epsilon * r)^2)
    Gaussian { epsilon: f32 },
}

impl RbfType {
    fn evaluate(&self, r: f32) -> f32 {
        match self {
            RbfType::ThinPlateSpline => {
                if r < 1e-10 {
                    0.0
                } else {
                    r * r * r.ln()
                }
            }
            RbfType::Multiquadric { epsilon } => (1.0 + (epsilon * r).powi(2)).sqrt(),
            RbfType::Gaussian { epsilon } => (-(epsilon * r).powi(2)).exp(),
        }
    }
}

/// RBF Interpolator for scattered 3D points
pub struct RbfInterpolator {
    points: Vec<[f32; 2]>,
    weights: DVector<f32>,
    rbf_type: RbfType,
}

impl RbfInterpolator {
    /// Create a new RBF interpolator from a set of (x, y, z) points
    pub fn new(points: &[[f32; 3]], rbf_type: RbfType) -> Result<Self, String> {
        let n = points.len();
        if n < 3 {
            return Err("At least 3 points are required for RBF interpolation".to_string());
        }

        let mut a = DMatrix::zeros(n, n);
        let mut b = DVector::zeros(n);

        let xy_points: Vec<[f32; 2]> = points.iter().map(|p| [p[0], p[1]]).collect();

        for i in 0..n {
            b[i] = points[i][2];
            for j in 0..n {
                let dx = xy_points[i][0] - xy_points[j][0];
                let dy = xy_points[i][1] - xy_points[j][1];
                let r = (dx * dx + dy * dy).sqrt();
                a[(i, j)] = rbf_type.evaluate(r);
            }
        }

        // Add a small regularization term to the diagonal for stability
        for i in 0..n {
            a[(i, i)] += 1e-6;
        }

        let weights = a
            .lu()
            .solve(&b)
            .ok_or("Failed to solve RBF linear system")?;

        Ok(Self {
            points: xy_points,
            weights,
            rbf_type,
        })
    }

    /// Evaluate the interpolator at a given (x, y) point
    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        let mut sum = 0.0;
        for (i, p) in self.points.iter().enumerate() {
            let dx = x - p[0];
            let dy = y - p[1];
            let r = (dx * dx + dy * dy).sqrt();
            sum += self.weights[i] * self.rbf_type.evaluate(r);
        }
        sum
    }

    /// Generate a regular grid mesh from the interpolator
    pub fn generate_mesh(
        &self,
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
        steps_x: usize,
        steps_y: usize,
    ) -> Mesh {
        let mut vertices = Vec::with_capacity(steps_x * steps_y);
        let mut indices = Vec::new();

        let dx = if steps_x > 1 {
            (max_x - min_x) / (steps_x - 1) as f32
        } else {
            0.0
        };
        let dy = if steps_y > 1 {
            (max_y - min_y) / (steps_y - 1) as f32
        } else {
            0.0
        };

        for j in 0..steps_y {
            let y = min_y + j as f32 * dy;
            for i in 0..steps_x {
                let x = min_x + i as f32 * dx;
                let z = self.evaluate(x, y);
                vertices.push([x, y, z]);
            }
        }

        for j in 0..(steps_y - 1) {
            for i in 0..(steps_x - 1) {
                let p0 = (j * steps_x + i) as u32;
                let p1 = (j * steps_x + i + 1) as u32;
                let p2 = ((j + 1) * steps_x + i) as u32;
                let p3 = ((j + 1) * steps_x + i + 1) as u32;

                // Triangle 1
                indices.push(p0);
                indices.push(p1);
                indices.push(p2);

                // Triangle 2
                indices.push(p1);
                indices.push(p3);
                indices.push(p2);
            }
        }

        let mut mesh = Mesh::new(vertices, indices);
        mesh.compute_normals();
        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rbf_interpolation_linear() {
        // Simple plane: z = x + y
        let points = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 2.0],
        ];

        // Thin Plate Spline
        let interpolator = RbfInterpolator::new(&points, RbfType::ThinPlateSpline)
            .expect("Failed to create interpolator");

        // Test at original points (tolerance slightly higher due to regularization)
        for p in &points {
            let z = interpolator.evaluate(p[0], p[1]);
            assert!(
                (z - p[2]).abs() < 1e-3,
                "TPS evaluation at point {:?} failed: got {}, expected {}",
                p,
                z,
                p[2]
            );
        }

        // Multiquadric
        let interpolator_mq = RbfInterpolator::new(&points, RbfType::Multiquadric { epsilon: 1.0 })
            .expect("Failed to create interpolator");
        for p in &points {
            let z = interpolator_mq.evaluate(p[0], p[1]);
            assert!(
                (z - p[2]).abs() < 1e-3,
                "MQ evaluation at point {:?} failed: got {}, expected {}",
                p,
                z,
                p[2]
            );
        }
    }

    #[test]
    fn test_rbf_mesh_generation() {
        let points = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 2.0],
        ];

        let interpolator = RbfInterpolator::new(&points, RbfType::ThinPlateSpline)
            .expect("Failed to create interpolator");
        let mesh = interpolator.generate_mesh(0.0, 1.0, 0.0, 1.0, 5, 5);

        assert_eq!(mesh.vertices.len(), 25);
        assert_eq!(mesh.indices.len(), (5 - 1) * (5 - 1) * 6);
        assert!(mesh.normals.is_some());
    }
}
