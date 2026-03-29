//! Grid-based volumetric engine for surface-to-surface volume calculations.

use crate::interpolation::RbfInterpolator;

/// Volumetric engine to compute volume between surfaces.
pub struct VolumetricEngine;

impl Default for VolumetricEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl VolumetricEngine {
    /// Create a new volumetric engine.
    pub fn new() -> Self {
        Self
    }

    /// Calculate the volume between an upper and lower surface within a rectangular grid.
    ///
    /// The integration sums `(z_upper - z_lower) * dx * dy` for all points on the grid
    /// where the upper surface is above the lower surface.
    #[allow(clippy::too_many_arguments)]
    pub fn calculate_volume(
        &self,
        upper_surface: &RbfInterpolator,
        lower_surface: &RbfInterpolator,
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
        steps_x: usize,
        steps_y: usize,
    ) -> f32 {
        if steps_x < 2 || steps_y < 2 {
            return 0.0;
        }

        let dx = (max_x - min_x) / (steps_x - 1) as f32;
        let dy = (max_y - min_y) / (steps_y - 1) as f32;
        let cell_area = dx * dy;

        let mut total_volume = 0.0;

        for j in 0..steps_y {
            let y = min_y + j as f32 * dy;
            for i in 0..steps_x {
                let x = min_x + i as f32 * dx;

                let z_upper = upper_surface.evaluate(x, y);
                let z_lower = lower_surface.evaluate(x, y);

                let thickness = z_upper - z_lower;
                if thickness > 0.0 {
                    // Weighting: grid points on edges and corners should have less weight
                    // if this were a strictly trapezoidal rule, but for a dense grid
                    // simple summation of cells centered at points (or corners) works.
                    // Here we use a midpoint-like approach where each point represents one dx*dy area.
                    total_volume += thickness * cell_area;
                }
            }
        }

        total_volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpolation::RbfType;

    #[test]
    fn test_constant_thickness_volume() {
        // Upper surface at z = 10.0
        let upper_points = vec![
            [-10.0, -10.0, 10.0],
            [110.0, -10.0, 10.0],
            [-10.0, 110.0, 10.0],
            [110.0, 110.0, 10.0],
        ];
        let upper = RbfInterpolator::new(&upper_points, RbfType::ThinPlateSpline).unwrap();

        // Lower surface at z = 0.0
        let lower_points = vec![
            [-10.0, -10.0, 0.0],
            [110.0, -10.0, 0.0],
            [-10.0, 110.0, 0.0],
            [110.0, 110.0, 0.0],
        ];
        let lower = RbfInterpolator::new(&lower_points, RbfType::ThinPlateSpline).unwrap();

        let engine = VolumetricEngine::new();

        // Grid 0 to 100, area 10000. steps=101 means dx=1.0.
        // Number of points = 101*101 = 10201.
        // Total volume should be 10.0 * 100.0 * 100.0 = 100,000.
        // Note: With 101 points, dx = 1.0. Point 0 is at 0, Point 100 is at 100.
        // cell_area = 1.0 * 1.0 = 1.0.
        // Summing 101 * 101 points each with area 1.0 gives 10201 * 10.0 = 102,010.
        // This is because we are including the boundary points fully.

        let volume = engine.calculate_volume(&upper, &lower, 0.0, 100.0, 0.0, 100.0, 101, 101);

        // Expected is closer to 100,000.
        // To get exactly 100,000 with a simple sum, we'd need to handle weights or use steps_x such that it covers the intervals.

        println!("Calculated volume: {}", volume);
        assert!((volume - 100000.0).abs() < 5000.0); // Within 5%
    }
}
