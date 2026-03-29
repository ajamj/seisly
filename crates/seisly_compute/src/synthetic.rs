//! Synthetic Data Generator
//!
//! Generates realistic synthetic seismic volumes, well logs, and horizon picks
//! for testing and demonstration purposes without requiring real data.

use rand::Rng;
use std::f32::consts::PI;

/// Synthetic seismic volume generator
pub struct SyntheticSeismic {
    pub inline_count: usize,
    pub crossline_count: usize,
    pub sample_count: usize,
    pub inline_range: (i32, i32),
    pub crossline_range: (i32, i32),
    pub sample_rate_ms: f32,
}

impl SyntheticSeismic {
    pub fn new(inline_count: usize, crossline_count: usize, sample_count: usize) -> Self {
        Self {
            inline_count,
            crossline_count,
            sample_count,
            inline_range: (0, inline_count as i32),
            crossline_range: (0, crossline_count as i32),
            sample_rate_ms: 4.0, // 4ms sample rate
        }
    }

    /// Generate seismic volume with realistic features
    pub fn generate(&self) -> Vec<f32> {
        let mut data = vec![0.0f32; self.inline_count * self.crossline_count * self.sample_count];
        let mut rng = rand::thread_rng();

        // Generate background noise
        for val in data.iter_mut() {
            *val = rng.gen_range(-0.1..=0.1);
        }

        // Add reflectors (horizontal layers)
        let reflectors = vec![
            (100, 0.8), // Sample 100, amplitude 0.8
            (200, 0.6), // Sample 200, amplitude 0.6
            (300, 0.9), // Sample 300, amplitude 0.9
            (400, 0.7), // Sample 400, amplitude 0.7
        ];

        for (reflector_sample, amplitude) in reflectors {
            self.add_reflector(&mut data, reflector_sample, amplitude);
        }

        // Add fault (vertical displacement)
        self.add_fault(&mut data, 250, 50); // Fault at inline 250, 50 samples throw

        // Add anticline structure
        self.add_fold(&mut data, 150, 20);

        data
    }

    fn add_reflector(&self, data: &mut [f32], sample: usize, amplitude: f32) {
        let wavelet = self.ricker_wavelet(sample, 35.0); // 35Hz dominant frequency

        for il in 0..self.inline_count {
            for xl in 0..self.crossline_count {
                let idx = (il * self.crossline_count + xl) * self.sample_count;
                for (i, &wavelet_val) in wavelet.iter().enumerate() {
                    if idx + i < data.len() {
                        data[idx + i] += amplitude * wavelet_val;
                    }
                }
            }
        }
    }

    fn add_fault(&self, data: &mut [f32], fault_inline: usize, throw_samples: usize) {
        let mut rng = rand::thread_rng();

        for il in 0..self.inline_count {
            for xl in 0..self.crossline_count {
                let base_idx = (il * self.crossline_count + xl) * self.sample_count;

                // Apply displacement near fault
                if il >= fault_inline - 20 && il <= fault_inline + 20 {
                    let distance = (il as i32 - fault_inline as i32).abs() as f32;
                    let displacement = ((20.0 - distance) / 20.0 * throw_samples as f32) as usize;

                    // Shift samples below fault
                    for s in (displacement..self.sample_count).rev() {
                        let idx = base_idx + s;
                        if idx < data.len() && s >= displacement {
                            let src_idx = base_idx + s - displacement;
                            if src_idx < data.len() {
                                data[idx] = data[src_idx] + rng.gen_range(-0.05..=0.05);
                            }
                        }
                    }
                }
            }
        }
    }

    fn add_fold(&self, data: &mut [f32], _center_sample: usize, amplitude: usize) {
        for il in 0..self.inline_count {
            for xl in 0..self.crossline_count {
                let base_idx = (il * self.crossline_count + xl) * self.sample_count;

                // Calculate fold amplitude based on position (anticline)
                let il_center = self.inline_count as f32 / 2.0;
                let xl_center = self.crossline_count as f32 / 2.0;
                let dist_il = (il as f32 - il_center).abs() / il_center;
                let dist_xl = (xl as f32 - xl_center).abs() / xl_center;
                let fold = (1.0 - dist_il.max(dist_xl)) * amplitude as f32;

                // Shift reflectors
                for s in 0..self.sample_count {
                    let idx = base_idx + s;
                    if idx < data.len() {
                        let shift = fold as usize;
                        if s >= shift {
                            let src_idx = base_idx + s - shift;
                            if src_idx < data.len() {
                                data[idx] = data[src_idx];
                            }
                        }
                    }
                }
            }
        }
    }

    fn ricker_wavelet(&self, _center_sample: usize, dominant_freq: f32) -> Vec<f32> {
        let dt = self.sample_rate_ms / 1000.0;
        let half_length = 100.0; // ms
        let samples_each_side = (half_length / (dt * 1000.0)) as usize;

        let mut wavelet = vec![0.0f32; samples_each_side * 2 + 1];
        let pi_dt_freq = PI * dominant_freq * dt;

        for (i, val) in wavelet.iter_mut().enumerate() {
            let t = (i as i32 - samples_each_side as i32) as f32 * dt;
            let pi_ft = pi_dt_freq * t;
            *val = (1.0 - 2.0 * pi_ft * pi_ft) * (-pi_ft * pi_ft).exp();
        }

        // Normalize
        let max_val = wavelet.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
        for val in &mut wavelet {
            *val /= max_val;
        }

        wavelet
    }
}

/// Synthetic well log generator
pub struct SyntheticWellLog {
    pub well_name: String,
    pub x: f64,
    pub y: f64,
    pub elevation: f32,
    pub total_depth: f32,
    pub sample_interval: f32,
}

impl SyntheticWellLog {
    pub fn new(name: &str, x: f64, y: f64, elevation: f32, total_depth: f32) -> Self {
        Self {
            well_name: name.to_string(),
            x,
            y,
            elevation,
            total_depth,
            sample_interval: 0.5, // 0.5m sample interval
        }
    }

    /// Generate GR (Gamma Ray) log
    pub fn generate_gr(&self) -> (Vec<f32>, Vec<f32>) {
        let mut rng = rand::thread_rng();
        let num_samples = (self.total_depth / self.sample_interval) as usize;
        let mut depths = Vec::with_capacity(num_samples);
        let mut gr_values = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let depth = i as f32 * self.sample_interval;
            depths.push(depth);

            // Generate GR with geological patterns
            let mut gr = 50.0; // Base GR

            // Add shale layers
            if (1000.0..=1200.0).contains(&depth) {
                gr += 40.0 + rng.gen_range(-10.0..=10.0);
            }
            if (1800.0..=2000.0).contains(&depth) {
                gr += 50.0 + rng.gen_range(-15.0..=15.0);
            }

            // Add sand layers (low GR)
            if (1500.0..=1700.0).contains(&depth) {
                gr = 30.0 + rng.gen_range(-5.0..=5.0);
            }
            if (2200.0..=2400.0).contains(&depth) {
                gr = 25.0 + rng.gen_range(-5.0..=5.0);
            }

            // Add trend (GR increases with depth due to compaction)
            gr += depth * 0.01;

            // Add noise
            gr += rng.gen_range(-3.0..=3.0);

            gr_values.push(gr.clamp(0.0, 150.0));
        }

        (depths, gr_values)
    }

    /// Generate DT (Sonic) log
    pub fn generate_dt(&self) -> (Vec<f32>, Vec<f32>) {
        let mut rng = rand::thread_rng();
        let num_samples = (self.total_depth / self.sample_interval) as usize;
        let mut depths = Vec::with_capacity(num_samples);
        let mut dt_values = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let depth = i as f32 * self.sample_interval;
            depths.push(depth);

            // Base DT (compaction trend)
            let mut dt = 300.0 - depth * 0.05;

            // Shale layers (higher DT)
            if (1000.0..=1200.0).contains(&depth) {
                dt += 50.0 + rng.gen_range(-10.0..=10.0);
            }
            if (1800.0..=2000.0).contains(&depth) {
                dt += 60.0 + rng.gen_range(-15.0..=15.0);
            }

            // Sand layers (lower DT)
            if (1500.0..=1700.0).contains(&depth) {
                dt = 200.0 + rng.gen_range(-10.0..=10.0);
            }
            if (2200.0..=2400.0).contains(&depth) {
                dt = 180.0 + rng.gen_range(-10.0..=10.0);
            }

            // Add noise
            dt += rng.gen_range(-5.0..=5.0);

            dt_values.push(dt.clamp(100.0, 400.0));
        }

        (depths, dt_values)
    }

    /// Generate RHOB (Density) log
    pub fn generate_rhob(&self) -> (Vec<f32>, Vec<f32>) {
        let mut rng = rand::thread_rng();
        let num_samples = (self.total_depth / self.sample_interval) as usize;
        let mut depths = Vec::with_capacity(num_samples);
        let mut rhob_values = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let depth = i as f32 * self.sample_interval;
            depths.push(depth);

            // Base RHOB (compaction trend)
            let mut rhob = 2.0 + depth * 0.0003;

            // Shale layers (lower density)
            if (1000.0..=1200.0).contains(&depth) {
                rhob += 0.2 + rng.gen_range(-0.1..=0.1);
            }
            if (1800.0..=2000.0).contains(&depth) {
                rhob += 0.3 + rng.gen_range(-0.1..=0.1);
            }

            // Sand layers (higher density)
            if (1500.0..=1700.0).contains(&depth) {
                rhob = 2.4 + rng.gen_range(-0.1..=0.1);
            }
            if (2200.0..=2400.0).contains(&depth) {
                rhob = 2.5 + rng.gen_range(-0.1..=0.1);
            }

            // Add noise
            rhob += rng.gen_range(-0.05..=0.05);

            rhob_values.push(rhob.clamp(1.5, 3.0));
        }

        (depths, rhob_values)
    }
}

/// Synthetic horizon picks generator
pub struct SyntheticHorizonPicks {
    pub horizon_name: String,
    pub base_sample: usize,
    pub variation: usize,
}

impl SyntheticHorizonPicks {
    pub fn new(name: &str, base_sample: usize, variation: usize) -> Self {
        Self {
            horizon_name: name.to_string(),
            base_sample,
            variation,
        }
    }

    /// Generate picks for a grid
    pub fn generate(&self, inline_count: usize, crossline_count: usize) -> Vec<[f32; 3]> {
        let mut rng = rand::thread_rng();
        let mut picks = Vec::new();

        // Generate picks on a regular grid
        let step = 20; // Pick every 20 samples

        for il in (0..inline_count).step_by(step) {
            for xl in (0..crossline_count).step_by(step) {
                // Add structural variation (anticline)
                let il_center = inline_count as f32 / 2.0;
                let xl_center = crossline_count as f32 / 2.0;
                let dist =
                    ((il as f32 - il_center).powi(2) + (xl as f32 - xl_center).powi(2)).sqrt();
                let structural_shift = (dist / il_center * self.variation as f32 * 0.5) as i32;

                // Add random variation
                let random_shift =
                    rng.gen_range(-(self.variation as i32 / 2)..=(self.variation as i32 / 2));

                let sample =
                    (self.base_sample as i32 - structural_shift + random_shift).max(0) as usize;

                picks.push([il as f32, xl as f32, sample as f32]);
            }
        }

        picks
    }

    /// Generate fault stick picks
    pub fn generate_fault_sticks(
        &self,
        fault_inline: usize,
        stick_count: usize,
    ) -> Vec<Vec<[f32; 3]>> {
        let mut rng = rand::thread_rng();
        let mut sticks = Vec::new();

        for i in 0..stick_count {
            let mut stick = Vec::new();
            let crossline = (i * 50) % 500; // Spread sticks across crossline

            // Generate pick points along the fault
            for sample in (0..400).step_by(50) {
                let inline = fault_inline as f32 + rng.gen_range(-10.0..=10.0);
                let sample_depth = sample as f32 + rng.gen_range(-20.0..=20.0);
                stick.push([inline, crossline as f32, sample_depth]);
            }

            sticks.push(stick);
        }

        sticks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthetic_seismic_generation() {
        let seismic = SyntheticSeismic::new(100, 100, 500);
        let data = seismic.generate();

        assert_eq!(data.len(), 100 * 100 * 500);
        // Check that we have signal (not just noise)
        assert!(data.iter().any(|&x| x.abs() > 0.2));
    }

    #[test]
    fn test_synthetic_well_log_generation() {
        let well = SyntheticWellLog::new("Test Well", 500000.0, 1000000.0, 100.0, 3000.0);
        let (depths, gr) = well.generate_gr();

        assert_eq!(depths.len(), gr.len());
        assert_eq!(depths[0], 0.0);
        assert!(depths[depths.len() - 1] >= 2999.0);
        assert!(gr.iter().any(|&x| x > 70.0)); // Should have shale
        assert!(gr.iter().any(|&x| x < 40.0)); // Should have sand
    }

    #[test]
    fn test_synthetic_horizon_picks() {
        let horizon = SyntheticHorizonPicks::new("Test Horizon", 200, 30);
        let picks = horizon.generate(100, 100);

        assert!(!picks.is_empty());
        assert!(picks.iter().all(|p| p[0] >= 0.0 && p[0] < 100.0));
        assert!(picks.iter().all(|p| p[1] >= 0.0 && p[1] < 100.0));
        assert!(picks.iter().all(|p| p[2] > 150.0 && p[2] < 250.0)); // Around base with variation
    }
}
