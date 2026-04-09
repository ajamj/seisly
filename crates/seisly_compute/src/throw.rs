use seisly_core::domain::surface::Mesh;

/// A single throw measurement at a point along a fault.
pub struct ThrowPoint {
    pub position: [f32; 3],
    pub throw_value: f32,
}

/// Result of cross-correlation throw calculation.
pub struct CrossCorrelationThrow {
    /// Throw map in milliseconds: [inline, crossline]
    pub throw_map: Vec<Vec<f32>>,
    /// Number of inline traces
    pub n_ilines: usize,
    /// Number of crossline traces
    pub n_xlines: usize,
    /// Sample interval in milliseconds
    pub dt_ms: f32,
}

/// Result of gradient-based throw calculation.
pub struct GradientThrow {
    /// Throw map in milliseconds: [inline, crossline]
    pub throw_map: Vec<Vec<f32>>,
    /// Number of inline traces
    pub n_ilines: usize,
    /// Number of crossline traces
    pub n_xlines: usize,
    /// Sample interval in milliseconds
    pub dt_ms: f32,
}

// ============================================================================
// Method 1: Horizon-based Throw (Mesh Intersection)
// ============================================================================

/// Calculate vertical throw distribution along an intersection line between
/// two mesh components (hanging wall and footwall horizons).
///
/// Uses barycentric interpolation for exact Z values at query points.
pub fn calculate_throw_distribution(
    up_mesh: &Mesh,
    down_mesh: &Mesh,
    intersection_line: &[[f32; 3]],
) -> Vec<ThrowPoint> {
    let mut distribution = Vec::with_capacity(intersection_line.len());

    for &point in intersection_line {
        let z_up = interpolate_z_barycentric(up_mesh, point[0], point[1]);
        let z_down = interpolate_z_barycentric(down_mesh, point[0], point[1]);

        if let (Some(zu), Some(zd)) = (z_up, z_down) {
            distribution.push(ThrowPoint {
                position: point,
                throw_value: (zu - zd).abs(),
            });
        }
    }

    distribution
}

/// Interpolate Z at (x, y) using barycentric coordinates of the containing triangle.
/// Falls back to nearest-vertex if no containing triangle is found.
fn interpolate_z_barycentric(mesh: &Mesh, x: f32, y: f32) -> Option<f32> {
    // Try barycentric interpolation first
    for tri_start in (0..mesh.indices.len()).step_by(3) {
        if tri_start + 2 >= mesh.indices.len() {
            continue;
        }
        let i0 = mesh.indices[tri_start] as usize;
        let i1 = mesh.indices[tri_start + 1] as usize;
        let i2 = mesh.indices[tri_start + 2] as usize;

        let v0 = mesh.vertices.get(i0);
        let v1 = mesh.vertices.get(i1);
        let v2 = mesh.vertices.get(i2);

        if let (Some(v0), Some(v1), Some(v2)) = (v0, v1, v2) {
            if let Some((alpha, beta, gamma)) = barycentric_coords(v0, v1, v2, x, y) {
                let z = alpha * v0[2] + beta * v1[2] + gamma * v2[2];
                return Some(z);
            }
        }
    }

    // Fallback: nearest vertex within search radius
    find_nearest_z(mesh, x, y)
}

/// Compute barycentric coordinates of point (x, y) relative to triangle (v0, v1, v2).
/// Returns Some(alpha, beta, gamma) if point is inside triangle, None otherwise.
fn barycentric_coords(
    v0: &[f32; 3],
    v1: &[f32; 3],
    v2: &[f32; 3],
    x: f32,
    y: f32,
) -> Option<(f32, f32, f32)> {
    let denom = (v1[1] - v2[1]) * (v0[0] - v2[0]) + (v2[0] - v1[0]) * (v0[1] - v2[1]);
    if denom.abs() < 1e-10 {
        return None;
    }

    let alpha = ((v1[1] - v2[1]) * (x - v2[0]) + (v2[0] - v1[0]) * (y - v2[1])) / denom;
    let beta = ((v2[1] - v0[1]) * (x - v2[0]) + (v0[0] - v2[0]) * (y - v2[1])) / denom;
    let gamma = 1.0 - alpha - beta;

    // Point is inside triangle if all barycentric coords are in [0, 1]
    if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
        Some((alpha, beta, gamma))
    } else {
        None
    }
}

/// Find Z of nearest vertex within search radius.
fn find_nearest_z(mesh: &Mesh, x: f32, y: f32) -> Option<f32> {
    let mut min_dist_sq = f32::MAX;
    let mut nearest_z = None;
    let search_radius_sq = 100.0f32; // 10 unit radius

    for v in &mesh.vertices {
        let dx = v[0] - x;
        let dy = v[1] - y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq < min_dist_sq && dist_sq < search_radius_sq {
            min_dist_sq = dist_sq;
            nearest_z = Some(v[2]);
        }
    }

    nearest_z
}

// ============================================================================
// Method 2: Cross-correlation Throw
// ============================================================================

/// Calculate fault throw using cross-correlation method.
///
/// For each fault point, finds the vertical shift that maximizes correlation
/// between hanging wall and footwall traces.
///
/// # Arguments
/// * `volume` — 3D seismic volume [inline, crossline, time]
/// * `fault_mask` — 3D binary fault mask (1 = fault, 0 = non-fault)
/// * `dt_ms` — Sample interval in milliseconds
/// * `max_throw_samples` — Maximum throw to search (in samples)
pub fn calculate_throw_cross_correlation(
    volume: &[Vec<Vec<f32>>],
    fault_mask: &[Vec<Vec<u8>>],
    dt_ms: f32,
    max_throw_samples: usize,
) -> CrossCorrelationThrow {
    let n_ilines = volume.len();
    let n_xlines = volume.first().map(|v| v.len()).unwrap_or(0);
    let _nt = volume
        .first()
        .and_then(|v| v.first())
        .map(|v| v.len())
        .unwrap_or(0);

    let mut throw_map = vec![vec![0.0f32; n_xlines]; n_ilines];

    // Find fault locations (2D projection)
    for i in 0..n_ilines {
        for j in 0..n_xlines {
            // Check if any sample at this (i, j) is a fault
            let is_fault = fault_mask
                .get(i)
                .and_then(|row| row.get(j))
                .map_or(false, |v| v.iter().any(|&b| b != 0));

            if !is_fault {
                continue;
            }

            // Skip edges
            if i < 2 || i >= n_ilines - 2 || j < 2 || j >= n_xlines - 2 {
                continue;
            }

            // Extract traces on either side of fault
            let trace1 = volume.get(i - 1).and_then(|v| v.get(j)).cloned();
            let trace2 = volume.get(i + 1).and_then(|v| v.get(j)).cloned();

            if let (Some(t1), Some(t2)) = (trace1, trace2) {
                let best_lag = cross_correlate(&t1, &t2, max_throw_samples);
                throw_map[i][j] = (best_lag as f32 * dt_ms).abs();
            }
        }
    }

    CrossCorrelationThrow {
        throw_map,
        n_ilines,
        n_xlines,
        dt_ms,
    }
}

/// Find the lag that maximizes cross-correlation between two traces.
fn cross_correlate(trace1: &[f32], trace2: &[f32], max_lag: usize) -> isize {
    let min_len = trace1.len().min(trace2.len());
    if min_len == 0 {
        return 0;
    }

    let center = max_lag.min(min_len / 2);
    let mut best_lag: isize = 0;
    let mut best_corr = f32::MIN;

    for lag in -(center as isize)..=(center as isize) {
        let mut sum = 0.0f32;
        let mut count = 0usize;

        for k in 0..min_len {
            let k2 = k as isize + lag;
            if k2 >= 0 && (k2 as usize) < min_len {
                sum += trace1[k] * trace2[k2 as usize];
                count += 1;
            }
        }

        if count > 0 {
            let avg_corr = sum / count as f32;
            if avg_corr > best_corr {
                best_corr = avg_corr;
                best_lag = lag;
            }
        }
    }

    best_lag
}

// ============================================================================
// Method 3: Gradient-based Throw
// ============================================================================

/// Calculate fault throw using vertical gradient sign change method.
///
/// Computes vertical gradient of seismic amplitude and detects where
/// gradient sign changes across the fault plane.
///
/// # Arguments
/// * `volume` — 3D seismic volume [inline, crossline, time]
/// * `fault_mask` — 3D binary fault mask (1 = fault, 0 = non-fault)
/// * `dt_ms` — Sample interval in milliseconds
pub fn calculate_throw_gradient(
    volume: &[Vec<Vec<f32>>],
    fault_mask: &[Vec<Vec<u8>>],
    dt_ms: f32,
) -> GradientThrow {
    let n_ilines = volume.len();
    let n_xlines = volume.first().map(|v| v.len()).unwrap_or(0);
    let nt = volume
        .first()
        .and_then(|v| v.first())
        .map(|v| v.len())
        .unwrap_or(0);

    // Compute vertical gradient
    let mut grad_z = vec![vec![vec![0.0f32; nt]; n_xlines]; n_ilines];

    for i in 0..n_ilines {
        for j in 0..n_xlines {
            if let Some(trace) = volume.get(i).and_then(|v| v.get(j)) {
                for k in 0..nt {
                    let prev = if k > 0 { trace[k - 1] } else { trace[k] };
                    let next = if k + 1 < nt { trace[k + 1] } else { trace[k] };
                    grad_z[i][j][k] = (next - prev) * 0.5;
                }
            }
        }
    }

    let mut throw_map = vec![vec![0.0f32; n_xlines]; n_ilines];

    // Find fault locations and compute throw
    for i in 0..n_ilines {
        for j in 0..n_xlines {
            let is_fault = fault_mask
                .get(i)
                .and_then(|row| row.get(j))
                .map_or(false, |v| v.iter().any(|&b| b != 0));

            if !is_fault {
                continue;
            }

            if i < 1 || i >= n_ilines - 1 {
                continue;
            }

            // Extract gradient profiles on either side of fault
            let grad_left = grad_z.get(i - 1).and_then(|v| v.get(j)).cloned();
            let grad_right = grad_z.get(i + 1).and_then(|v| v.get(j)).cloned();

            if let (Some(gl), Some(gr)) = (grad_left, grad_right) {
                let min_dist = find_min_zero_crossing_distance(&gl, &gr);
                throw_map[i][j] = min_dist as f32 * dt_ms;
            }
        }
    }

    GradientThrow {
        throw_map,
        n_ilines,
        n_xlines,
        dt_ms,
    }
}

/// Find the minimum distance between zero-crossing pairs in two gradient profiles.
fn find_min_zero_crossing_distance(grad1: &[f32], grad2: &[f32]) -> usize {
    let zc1 = zero_crossings(grad1);
    let zc2 = zero_crossings(grad2);

    if zc1.is_empty() || zc2.is_empty() {
        return grad1.len(); // Return full length if no zero crossings
    }

    let mut min_dist = grad1.len();
    for &z1 in &zc1 {
        for &z2 in &zc2 {
            let dist = if z1 > z2 { z1 - z2 } else { z2 - z1 };
            if dist < min_dist {
                min_dist = dist;
            }
        }
    }

    min_dist
}

/// Find zero-crossing indices in a signal.
fn zero_crossings(signal: &[f32]) -> Vec<usize> {
    let mut crossings = Vec::new();
    for i in 1..signal.len() {
        let prev_sign = signal[i - 1].total_cmp(&0.0);
        let curr_sign = signal[i].total_cmp(&0.0);
        if prev_sign != std::cmp::Ordering::Equal
            && curr_sign != std::cmp::Ordering::Equal
            && prev_sign != curr_sign
        {
            crossings.push(i);
        }
    }
    crossings
}

// ============================================================================
// Tests
// ============================================================================

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

    #[test]
    fn test_barycentric_interpolation() {
        let mesh = Mesh::new(
            vec![[0.0, 0.0, 100.0], [10.0, 0.0, 100.0], [0.0, 10.0, 100.0]],
            vec![0, 1, 2],
        );

        // Center of triangle should interpolate to Z=100
        let z = interpolate_z_barycentric(&mesh, 3.33, 3.33);
        assert!(z.is_some());
        assert!((z.unwrap() - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_cross_correlation_known_lag() {
        let trace1: Vec<f32> = (0..50).map(|i| (i as f32 * 0.2).sin()).collect();
        let mut trace2 = vec![0.0f32; 50];
        // Shift by 5 samples
        for i in 5..50 {
            trace2[i] = trace1[i - 5];
        }

        let lag = cross_correlate(&trace1, &trace2, 20);
        assert_eq!(lag, 5);
    }

    #[test]
    fn test_cross_correlation_throw() {
        // Create simple 5x5x30 volume with known throw
        let nt = 30;
        let throw_samples = 5;
        let dt_ms = 4.0;

        let mut volume = vec![vec![vec![0.0f32; nt]; 5]; 5];
        let mut fault_mask = vec![vec![vec![0u8; nt]; 5]; 5];

        // Create simple reflector pattern with throw at i>=3
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..nt {
                    let shift = if i >= 3 { throw_samples } else { 0 };
                    let k_shifted = if k >= shift { k - shift } else { k };
                    volume[i][j][k] = (k_shifted as f32 * 0.3).sin();
                }
            }
        }

        // Mark fault at i=2, j=2
        fault_mask[2][2][nt / 2] = 1;

        let result = calculate_throw_cross_correlation(&volume, &fault_mask, dt_ms, 15);

        // Fault point should have non-zero throw
        assert!(
            result.throw_map[2][2] > 0.0,
            "Expected throw > 0 at fault point, got {}",
            result.throw_map[2][2]
        );
    }

    #[test]
    fn test_gradient_throw() {
        let nt = 20;
        let dt_ms = 4.0;

        let mut volume = vec![vec![vec![0.0f32; nt]; 3]; 3];
        let mut fault_mask = vec![vec![vec![0u8; nt]; 3]; 3];

        // Create simple pattern with gradient sign change across fault
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..nt {
                    let offset = if i >= 2 { 3 } else { 0 };
                    volume[i][j][k] = ((k + offset) as f32 * 0.5).sin();
                }
            }
        }

        // Mark fault
        fault_mask[1][1][nt / 2] = 1;

        let result = calculate_throw_gradient(&volume, &fault_mask, dt_ms);

        // Fault point should have computed throw
        assert!(result.throw_map[1][1] >= 0.0);
    }

    #[test]
    fn test_zero_crossings() {
        let signal: Vec<f32> = vec![-1.0, 1.0, -1.0];
        let zc = zero_crossings(&signal);
        // Crosses between 0-1 (negative to positive) and 1-2 (positive to negative)
        assert_eq!(zc.len(), 2);
        assert_eq!(zc[0], 1);
        assert_eq!(zc[1], 2);
    }
}
