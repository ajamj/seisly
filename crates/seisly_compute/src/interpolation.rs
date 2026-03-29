//! Radial Basis Function (RBF) interpolation for surface modeling

use nalgebra::{DMatrix, DVector, Matrix3, Vector3};
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
    points_uv: Vec<[f32; 2]>,
    weights: DVector<f32>,
    rbf_type: RbfType,
    centroid: Vector3<f32>,
    basis: Matrix3<f32>, // Columns are U, V, N basis vectors
}

impl RbfInterpolator {
    /// Create a new RBF interpolator from a set of (x, y, z) points
    /// Uses PCA to find the best-fitting plane and performs interpolation in that local coordinate system.
    pub fn new(points: &[[f32; 3]], rbf_type: RbfType) -> Result<Self, String> {
        let n = points.len();
        if n < 3 {
            return Err("At least 3 points are required for RBF interpolation".to_string());
        }

        // 1. Calculate centroid
        let mut centroid = Vector3::zeros();
        for p in points {
            centroid += Vector3::from_column_slice(p);
        }
        centroid /= n as f32;

        // 2. Compute covariance matrix for PCA
        let mut cov = Matrix3::zeros();
        for p in points {
            let relative = Vector3::from_column_slice(p) - centroid;
            cov += relative * relative.transpose();
        }
        cov /= (n - 1) as f32;

        // 3. SVD to find principal components
        let svd = cov.svd(true, true);
        let v_t = svd.v_t.ok_or("Failed to compute SVD")?;
        let v = v_t.transpose();

        // v columns are our new basis: [U, V, N]
        // SVD sorts singular values (and thus vectors) in descending order.
        // The last vector (smallest singular value) is the normal to the best-fit plane.
        let basis = v;

        // 4. Transform points to local UVN system
        let mut points_uv = Vec::with_capacity(n);
        let mut b = DVector::zeros(n);

        for i in 0..n {
            let p = Vector3::from_column_slice(&points[i]);
            let relative = p - centroid;
            let local = basis.transpose() * relative;

            points_uv.push([local.x, local.y]);
            b[i] = local.z; // We interpolate the 'N' component (distance from plane)
        }

        // 5. Solve RBF system in local UV space
        let mut a = DMatrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                let dx = points_uv[i][0] - points_uv[j][0];
                let dy = points_uv[i][1] - points_uv[j][1];
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
            points_uv,
            weights,
            rbf_type,
            centroid,
            basis,
        })
    }

    /// Evaluate the interpolator at a given (u, v) point in the local coordinate system
    /// returns the 'n' component (distance from the best-fit plane)
    pub fn evaluate_local(&self, u: f32, v: f32) -> f32 {
        let mut sum = 0.0;
        for (i, p) in self.points_uv.iter().enumerate() {
            let dx = u - p[0];
            let dy = v - p[1];
            let r = (dx * dx + dy * dy).sqrt();
            sum += self.weights[i] * self.rbf_type.evaluate(r);
        }
        sum
    }

    /// Evaluate the interpolator to find the 'z' coordinate for a given (x, y)
    /// Note: For high-angle planes, this might be multi-valued or poorly defined.
    /// It attempts to solve for 'z' such that the point (x, y, z) lies on the RBF surface.
    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        // We want to find z such that:
        // P = [x, y, z] - centroid
        // L = basis.T * P
        // L.z = evaluate_local(L.x, L.y)

        // This is:
        // (x - Cx)*Nx + (y - Cy)*Ny + (z - Cz)*Nz = f((x - Cx)*Ux + (y - Cy)*Uy + (z - Cz)*Uz, (x - Cx)*Vx + (y - Cy)*Vy + (z - Cz)*Vz)

        // For nearly horizontal planes, Nz is large, and we can solve this iteratively or assume z is close to a starting value.
        // For a vertical plane, Nz = 0, and this equation doesn't involve z on the left side.

        // Simple heuristic: if the plane is mostly horizontal, use the old 2.5D approach (which is what evaluate_local approximates if basis ~ Identity)
        // For now, let's implement a simple version that works for "reasonable" angles.
        // A better approach for 3D is to use generate_mesh_3d.

        let nz = self.basis[(2, 2)];
        if nz.abs() > 0.1 {
            // Solve iteratively or just approximate?
            // For now, let's keep the old behavior for evaluation if possible,
            // but the internal representation is already changed.

            // To be perfectly backward compatible with the 2.5D API, we'd need to solve the non-linear equation.
            // But if we just want it to work for "most" surfaces:

            let mut z = self.centroid.z;
            for _ in 0..5 {
                let p = Vector3::new(x, y, z) - self.centroid;
                let local = self.basis.transpose() * p;
                let n_target = self.evaluate_local(local.x, local.y);
                let diff = n_target - local.z;
                // local.z = p.dot(N) = (x-Cx)Nx + (y-Cy)Ny + (z-Cz)Nz
                // d(local.z)/dz = Nz
                // We want local.z + dz * Nz = n_target  => dz = (n_target - local.z) / Nz
                z += diff / nz;
            }
            z
        } else {
            // Vertical plane! For a given (x, y), there might be no z or many z.
            // Return centroid.z as a fallback.
            self.centroid.z
        }
    }

    /// Generate a regular grid mesh from the interpolator.
    /// This version uses the local coordinate system to support arbitrary orientations.
    pub fn generate_mesh_3d(&self, steps_u: usize, steps_v: usize) -> Mesh {
        let mut min_u = f32::MAX;
        let mut max_u = f32::MIN;
        let mut min_v = f32::MAX;
        let mut max_v = f32::MIN;

        for p in &self.points_uv {
            min_u = min_u.min(p[0]);
            max_u = max_u.max(p[0]);
            min_v = min_v.min(p[1]);
            max_v = max_v.max(p[1]);
        }

        // Add some padding
        let du = (max_u - min_u) * 0.1;
        let dv = (max_v - min_v) * 0.1;
        min_u -= du;
        max_u += du;
        min_v -= dv;
        max_v += dv;

        let mut vertices = Vec::with_capacity(steps_u * steps_v);
        let mut indices = Vec::new();

        let step_u = if steps_u > 1 {
            (max_u - min_u) / (steps_u - 1) as f32
        } else {
            0.0
        };
        let step_v = if steps_v > 1 {
            (max_v - min_v) / (steps_v - 1) as f32
        } else {
            0.0
        };

        for j in 0..steps_v {
            let v = min_v + j as f32 * step_v;
            for i in 0..steps_u {
                let u = min_u + i as f32 * step_u;
                let n = self.evaluate_local(u, v);

                // Transform back to global coordinates
                let local = Vector3::new(u, v, n);
                let global = self.basis * local + self.centroid;
                vertices.push([global.x, global.y, global.z]);
            }
        }

        for j in 0..(steps_v - 1) {
            for i in 0..(steps_u - 1) {
                let p0 = (j * steps_u + i) as u32;
                let p1 = (j * steps_u + i + 1) as u32;
                let p2 = ((j + 1) * steps_u + i) as u32;
                let p3 = ((j + 1) * steps_u + i + 1) as u32;

                indices.push(p0);
                indices.push(p1);
                indices.push(p2);
                indices.push(p1);
                indices.push(p3);
                indices.push(p2);
            }
        }

        let mut mesh = Mesh::new(vertices, indices);
        mesh.compute_normals();
        mesh
    }

    /// Generate a regular grid mesh from the interpolator (Compatibility version)
    pub fn generate_mesh(
        &self,
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
        steps_x: usize,
        steps_y: usize,
    ) -> Mesh {
        // For backward compatibility, we still implement this.
        // But if it's a vertical plane, it will still struggle because evaluate(x, y) is limited.

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

                indices.push(p0);
                indices.push(p1);
                indices.push(p2);
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

        // Test at original points
        for p in &points {
            let z = interpolator.evaluate(p[0], p[1]);
            assert!(
                (z - p[2]).abs() < 1e-2,
                "TPS evaluation at point {:?} failed: got {}, expected {}",
                p,
                z,
                p[2]
            );
        }
    }

    #[test]
    fn test_rbf_interpolation_vertical_plane() {
        // Vertical plane: x = 1.0 (varying in y and z)
        let points = vec![
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
        ];

        let interpolator = RbfInterpolator::new(&points, RbfType::ThinPlateSpline);
        assert!(
            interpolator.is_ok(),
            "Should handle vertical planes via PCA/Orientation"
        );

        let interpolator = interpolator.unwrap();

        // Use generate_mesh_3d for vertical planes
        let mesh = interpolator.generate_mesh_3d(2, 2);
        assert_eq!(mesh.vertices.len(), 4);

        // All vertices should have x close to 1.0
        for v in &mesh.vertices {
            assert!(
                (v[0] - 1.0).abs() < 1e-3,
                "Vertex x coordinate should be 1.0, got {}",
                v[0]
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
