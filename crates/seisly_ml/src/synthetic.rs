//! Synthetic Data Generator for Training
//!
//! Generates synthetic seismic-like data for training the horizon tracker.

use candle_core::{DType, Device, Result, Tensor};
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};

/// Configuration for synthetic data generation
#[derive(Debug, Clone)]
pub struct SyntheticConfig {
    /// Number of traces to generate
    pub num_traces: usize,
    /// Number of samples per trace
    pub num_samples: usize,
    /// Number of horizons to simulate
    pub num_horizons: usize,
    /// Noise level (standard deviation)
    pub noise_std: f32,
    /// Random seed for reproducibility
    pub seed: u64,
}

impl Default for SyntheticConfig {
    fn default() -> Self {
        Self {
            num_traces: 100,
            num_samples: 512,
            num_horizons: 3,
            noise_std: 0.1,
            seed: 42,
        }
    }
}

/// Synthetic data trainer for generating training datasets
pub struct SyntheticTrainer {
    config: SyntheticConfig,
    rng: rand::rngs::StdRng,
}

impl SyntheticTrainer {
    /// Create a new synthetic trainer with the given configuration
    pub fn new(config: SyntheticConfig) -> Self {
        let rng = rand::rngs::StdRng::seed_from_u64(config.seed);
        Self { config, rng }
    }

    /// Generate synthetic seismic data
    ///
    /// Returns a tensor of shape (num_traces, num_samples)
    pub fn generate_seismic(&mut self) -> Result<Tensor> {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut data = Vec::with_capacity(self.config.num_traces * self.config.num_samples);

        for _ in 0..self.config.num_traces {
            let mut trace = Vec::with_capacity(self.config.num_samples);
            let mut depth = 0.0;

            for i in 0..self.config.num_samples {
                // Simulate layered seismic response
                let layer_signal = self.generate_layer_signal(depth, i);
                let noise = normal.sample(&mut self.rng) * self.config.noise_std;
                trace.push(layer_signal + noise);
                depth += 0.001;
            }
            data.extend(trace);
        }

        Tensor::from_vec(
            data,
            (self.config.num_traces, self.config.num_samples),
            &Device::Cpu,
        )
    }

    /// Generate synthetic horizon labels
    ///
    /// Returns a tensor of shape (num_traces, num_horizons) with horizon depths
    pub fn generate_horizons(&mut self) -> Result<Tensor> {
        let normal = Normal::new(0.0, 10.0).unwrap();
        let mut horizons = Vec::with_capacity(self.config.num_traces * self.config.num_horizons);

        for _ in 0..self.config.num_traces {
            let mut base_depth = 100.0;
            for h in 0..self.config.num_horizons {
                let depth = base_depth + normal.sample(&mut self.rng) as f32;
                horizons.push(depth);
                base_depth += 50.0 + (h as f32) * 10.0;
            }
        }

        Tensor::from_vec(
            horizons,
            (self.config.num_traces, self.config.num_horizons),
            &Device::Cpu,
        )
    }

    /// Generate a complete training dataset
    ///
    /// Returns (seismic_data, horizon_labels)
    pub fn generate_dataset(&mut self) -> Result<(Tensor, Tensor)> {
        let seismic = self.generate_seismic()?;
        let horizons = self.generate_horizons()?;
        Ok((seismic, horizons))
    }

    /// Generate layer signal based on depth and sample index
    fn generate_layer_signal(&mut self, depth: f32, sample_idx: usize) -> f32 {
        // Simulate reflective layers at different depths
        let layer1 = (-((depth - 0.2) * 50.0).powi(2)).exp() * 0.8;
        let layer2 = (-((depth - 0.4) * 50.0).powi(2)).exp() * 0.6;
        let layer3 = (-((depth - 0.6) * 50.0).powi(2)).exp() * 0.4;

        // Add some frequency content
        let frequency = (sample_idx as f32 * 0.01).sin() * 0.2;

        layer1 + layer2 + layer3 + frequency
    }

    /// Add noise to existing seismic data
    pub fn add_noise(&mut self, data: &Tensor, noise_level: f32) -> Result<Tensor> {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let shape = data.dims();
        let total = shape.iter().product::<usize>();
        let mut noise_vec = Vec::with_capacity(total);

        for _ in 0..total {
            noise_vec.push(normal.sample(&mut self.rng) as f32 * noise_level);
        }

        let noise = Tensor::from_vec(noise_vec, shape, &Device::Cpu)?;
        data.add(&noise)
    }

    /// Get the current configuration
    pub fn config(&self) -> &SyntheticConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthetic_trainer_creation() {
        let config = SyntheticConfig::default();
        let trainer = SyntheticTrainer::new(config.clone());
        assert_eq!(trainer.config().num_traces, 100);
        assert_eq!(trainer.config().num_samples, 512);
    }

    #[test]
    fn test_generate_seismic() {
        let config = SyntheticConfig {
            num_traces: 10,
            num_samples: 64,
            ..Default::default()
        };
        let mut trainer = SyntheticTrainer::new(config);
        let seismic = trainer.generate_seismic().unwrap();
        assert_eq!(seismic.dims(), &[10, 64]);
    }

    #[test]
    fn test_generate_horizons() {
        let config = SyntheticConfig {
            num_traces: 10,
            num_horizons: 3,
            ..Default::default()
        };
        let mut trainer = SyntheticTrainer::new(config);
        let horizons = trainer.generate_horizons().unwrap();
        assert_eq!(horizons.dims(), &[10, 3]);
    }
}
