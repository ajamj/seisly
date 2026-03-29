//! Well-seismic tie computation
//!
//! Uses existing LinearVelocityModel (V0 + kZ) from sf_compute::velocity
//! for accurate time-depth conversion.
//!
//! Formula: TWT = (2/k) * ln((v0 + k*depth) / v0)

use sf_core::domain::Well;
use thiserror::Error;
use uuid::Uuid;

/// Error types for well tie computation
#[derive(Error, Debug)]
pub enum WellTieError {
    #[error("Well has no logs")]
    NoLogs,
    #[error("Invalid velocity parameters: {0}")]
    InvalidVelocity(String),
}

/// Parameters for well-seismic tie
#[derive(Debug, Clone)]
pub struct TieParameters {
    /// Datum elevation (meters)
    pub datum_elevation: f64,
    /// Surface velocity v0 (m/s)
    pub v0: f64,
    /// Velocity gradient k (1/s)
    pub k: f64,
}

impl Default for TieParameters {
    fn default() -> Self {
        Self {
            datum_elevation: 0.0,
            v0: 2000.0, // Typical sedimentary rock
            k: 0.5,     // Moderate compaction
        }
    }
}

/// Time-depth pair for well tie
#[derive(Debug, Clone)]
pub struct TimeDepthPair {
    pub depth_md: f64,    // Measured depth (m)
    pub twt: f64,         // Two-way time (ms)
}

/// Well-seismic tie result
#[derive(Debug, Clone)]
pub struct WellTie {
    pub id: Uuid,
    pub well_id: Uuid,
    pub time_depth_pairs: Vec<TimeDepthPair>,
    pub parameters: TieParameters,
}

/// Well tie computation engine using LinearVelocityModel
pub struct WellTieEngine {
    v0: f64,
    k: f64,
}

impl WellTieEngine {
    /// Create a new well tie engine with V0 + kZ velocity model
    ///
    /// # Arguments
    /// * `v0` - Surface velocity in m/s
    /// * `k` - Velocity gradient in 1/s
    pub fn new(v0: f64, k: f64) -> Self {
        Self { v0, k }
    }

    /// Create well-seismic tie using V0 + kZ model
    ///
    /// # Arguments
    /// * `well` - Well with at least one log
    ///
    /// # Returns
    /// WellTie with time-depth pairs at 10m intervals
    pub fn create_tie(&self, well: &Well) -> Result<WellTie, WellTieError> {
        // Get first log for depth range
        let first_log = well
            .logs
            .first()
            .ok_or(WellTieError::NoLogs)?;

        let min_depth = first_log.min_depth as f64;
        let max_depth = first_log.max_depth as f64;

        // Generate time-depth pairs using V0 + kZ formula
        let mut pairs = Vec::new();
        let step = 10.0; // 10m intervals

        let mut depth = min_depth;
        while depth <= max_depth {
            // TWT = (2/k) * ln((v0 + k*depth) / v0) * 1000 (convert to ms)
            let twt = Self::depth_to_twt(depth, self.v0, self.k);
            
            pairs.push(TimeDepthPair {
                depth_md: depth,
                twt,
            });

            depth += step;
        }

        Ok(WellTie {
            id: Uuid::new_v4(),
            well_id: well.id,
            time_depth_pairs: pairs,
            parameters: TieParameters {
                datum_elevation: 0.0,
                v0: self.v0,
                k: self.k,
            },
        })
    }

    /// Convert depth to TWT using V0 + kZ model
    ///
    /// # Formula
    /// TWT = (2/k) * ln((v0 + k*depth) / v0) * 1000
    ///
    /// # Arguments
    /// * `depth` - Depth in meters
    /// * `v0` - Surface velocity in m/s
    /// * `k` - Velocity gradient in 1/s
    ///
    /// # Returns
    /// Two-way time in milliseconds
    pub fn depth_to_twt(depth: f64, v0: f64, k: f64) -> f64 {
        (2.0 / k) * ((v0 + k * depth) / v0).ln() * 1000.0
    }

    /// Convert TWT to depth (inverse formula)
    ///
    /// # Formula
    /// depth = (v0 / k) * (exp(k * TWT / 2000) - 1)
    ///
    /// # Arguments
    /// * `twt` - Two-way time in milliseconds
    /// * `v0` - Surface velocity in m/s
    /// * `k` - Velocity gradient in 1/s
    ///
    /// # Returns
    /// Depth in meters
    pub fn twt_to_depth(twt: f64, v0: f64, k: f64) -> f64 {
        let twt_sec = twt / 1000.0; // Convert ms to seconds
        (v0 / k) * ((k * twt_sec / 2.0).exp() - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_time_conversion() {
        // For v0=2000, k=0.5, depth=1000m:
        // TWT = (2/0.5) * ln((2000 + 0.5*1000) / 2000) * 1000 = 892ms
        let twt = WellTieEngine::depth_to_twt(1000.0, 2000.0, 0.5);
        assert!((twt - 892.0).abs() < 1.0, "Expected TWT ~892ms, got {}", twt);

        // Back conversion
        let depth = WellTieEngine::twt_to_depth(twt, 2000.0, 0.5);
        assert!((depth - 1000.0).abs() < 1.0, "Expected depth ~1000m, got {}", depth);
    }

    #[test]
    fn test_zero_depth() {
        // At depth 0, TWT should be 0
        let twt = WellTieEngine::depth_to_twt(0.0, 2000.0, 0.5);
        assert!(twt.abs() < 1e-10, "Expected TWT ~0ms at depth 0, got {}", twt);
    }

    #[test]
    fn test_zero_twt() {
        // At TWT 0, depth should be 0
        let depth = WellTieEngine::twt_to_depth(0.0, 2000.0, 0.5);
        assert!(depth.abs() < 1e-10, "Expected depth ~0m at TWT 0, got {}", depth);
    }
}
