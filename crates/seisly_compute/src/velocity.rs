//! Linear Velocity Model (V0 + kZ)
//!
//! Provides conversion from Two-Way Time (TWT) to Depth (Z).

/// Linear velocity model for time-to-depth conversion.
/// Velocity at depth Z is given by V(Z) = v0 + k * Z.
/// The conversion for TWT is Z = (v0/k) * (exp(k * TWT / 2) - 1).
#[derive(Debug, Clone, Copy)]
pub struct LinearVelocityModel {
    /// Initial velocity at Z=0 (m/s)
    pub v0: f32,
    /// Acceleration gradient (s^-1)
    pub k: f32,
    /// Milliseconds per sample for unit conversion
    pub sample_rate_ms: f32,
    /// Start time offset (T0) in milliseconds
    pub start_time_ms: f32,
}

impl LinearVelocityModel {
    /// Create a new linear velocity model.
    pub fn new(v0: f32, k: f32, sample_rate_ms: f32, start_time_ms: f32) -> Self {
        Self {
            v0,
            k,
            sample_rate_ms,
            start_time_ms,
        }
    }

    /// Convert sample index to Depth (Z) in meters.
    pub fn sample_to_depth(&self, sample_idx: f32) -> f32 {
        let twt_sec = (self.start_time_ms + sample_idx * self.sample_rate_ms) / 1000.0;
        if self.k.abs() < 1e-6 {
            self.v0 * twt_sec / 2.0
        } else {
            (self.v0 / self.k) * ((self.k * twt_sec / 2.0).exp() - 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_velocity() {
        // v0 = 2000 m/s, k = 0, sample_rate = 4ms, start_time = 0
        let model = LinearVelocityModel::new(2000.0, 0.0, 4.0, 0.0);

        // At 0 samples (0ms), depth should be 0
        assert_eq!(model.sample_to_depth(0.0), 0.0);

        // At 250 samples (1000ms = 1s), depth should be 2000 * 1 / 2 = 1000m
        assert_eq!(model.sample_to_depth(250.0), 1000.0);
    }

    #[test]
    fn test_gradient_velocity() {
        // v0 = 2000 m/s, k = 0.5 s^-1, sample_rate = 4ms, start_time = 0
        let model = LinearVelocityModel::new(2000.0, 0.5, 4.0, 0.0);

        // At 0 samples (0ms), depth should be 0
        assert_eq!(model.sample_to_depth(0.0), 0.0);

        // At 500 samples (2000ms = 2s)
        // Z = (2000 / 0.5) * (exp(0.5 * 2 / 2) - 1)
        // Z = 4000 * (exp(0.5) - 1)
        // Z = 4000 * (1.648721 - 1) = 4000 * 0.648721 = 2594.884
        let depth = model.sample_to_depth(500.0);
        assert!((depth - 2594.884).abs() < 1e-3);
    }

    #[test]
    fn test_start_time_offset() {
        // v0 = 2000 m/s, k = 0, sample_rate = 4ms, start_time = 500ms
        let model = LinearVelocityModel::new(2000.0, 0.0, 4.0, 500.0);

        // At 0 samples (500ms = 0.5s), depth should be 2000 * 0.5 / 2 = 500m
        assert_eq!(model.sample_to_depth(0.0), 500.0);
    }
}
