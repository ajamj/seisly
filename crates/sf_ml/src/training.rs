//! Training Pipeline for Horizon Tracker
//!
//! Main training loop and configuration for the ML model.

use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{loss::mse, Optimizer, VarBuilder, VarMap};
use serde::{Deserialize, Serialize};

use crate::cnn::HorizonCNN;

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Number of training epochs
    pub epochs: usize,
    /// Batch size for training
    pub batch_size: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Weight decay for regularization
    pub weight_decay: f64,
    /// Early stopping patience (epochs without improvement)
    pub patience: usize,
    /// Minimum loss improvement to count as progress
    pub min_delta: f32,
    /// Random seed for reproducibility
    pub seed: u64,
    /// Device to train on ("cpu" or "cuda")
    pub device: String,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            epochs: 100,
            batch_size: 32,
            learning_rate: 1e-3,
            weight_decay: 1e-5,
            patience: 10,
            min_delta: 1e-4,
            seed: 42,
            device: "cpu".to_string(),
        }
    }
}

/// Training statistics
#[derive(Debug, Clone, Default)]
pub struct TrainingStats {
    /// Loss history per epoch
    pub loss_history: Vec<f32>,
    /// Best loss achieved
    pub best_loss: f32,
    /// Epoch of best loss
    pub best_epoch: usize,
    /// Final loss
    pub final_loss: f32,
    /// Whether early stopping was triggered
    pub early_stopped: bool,
    /// Total training time in seconds
    pub training_time: f64,
}

/// Main trainer for horizon tracking model
pub struct Trainer {
    config: TrainingConfig,
    model: HorizonCNN,
    varmap: VarMap,
    stats: TrainingStats,
}

impl Trainer {
    /// Create a new trainer with the given configuration
    pub fn new(config: TrainingConfig) -> Result<Self> {
        let device = Self::get_device(&config.device)?;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

        let model = HorizonCNN::new(vb)?;

        Ok(Self {
            config,
            model,
            varmap,
            stats: TrainingStats::default(),
        })
    }

    /// Train the model on the provided dataset
    ///
    /// Arguments:
    /// - seismic_data: Input seismic tensor (num_traces, num_samples)
    /// - horizon_labels: Target horizon depths (num_traces, num_horizons)
    pub fn train(&mut self, seismic_data: &Tensor, horizon_labels: &Tensor) -> Result<&TrainingStats> {
        let start_time = std::time::Instant::now();
        let device = seismic_data.device();

        let mut optimizer = candle_nn::AdamW::new(
            self.varmap.all_vars(),
            candle_nn::ParamsAdamW {
                lr: self.config.learning_rate,
                beta1: 0.9,
                beta2: 0.999,
                eps: 1e-8,
                weight_decay: self.config.weight_decay,
            },
        )?;

        let num_samples = seismic_data.dims()[0];
        let mut best_loss = f32::MAX;
        let mut patience_counter = 0;

        self.stats.loss_history.clear();

        for epoch in 0..self.config.epochs {
            // Shuffle data (simplified - in production use proper dataloader)
            let epoch_loss = self.train_epoch(
                seismic_data,
                horizon_labels,
                num_samples,
                &mut optimizer,
            )?;

            self.stats.loss_history.push(epoch_loss);

            // Check for improvement
            if epoch_loss < best_loss - self.config.min_delta {
                best_loss = epoch_loss;
                self.stats.best_loss = best_loss;
                self.stats.best_epoch = epoch;
                patience_counter = 0;
            } else {
                patience_counter += 1;
            }

            // Early stopping check
            if patience_counter >= self.config.patience {
                self.stats.early_stopped = true;
                break;
            }
        }

        self.stats.final_loss = *self.stats.loss_history.last().unwrap_or(&0.0);
        self.stats.training_time = start_time.elapsed().as_secs_f64();

        Ok(&self.stats)
    }

    /// Train for one epoch
    fn train_epoch(
        &mut self,
        seismic_data: &Tensor,
        horizon_labels: &Tensor,
        num_samples: usize,
        optimizer: &mut candle_nn::AdamW,
    ) -> Result<f32> {
        let mut total_loss = 0.0;
        let mut batch_count = 0;

        for batch_start in (0..num_samples).step_by(self.config.batch_size) {
            let batch_end = (batch_start + self.config.batch_size).min(num_samples);
            let batch_size = batch_end - batch_start;

            // Extract batch
            let seismic_batch = seismic_data.narrow(0, batch_start, batch_size)?;
            let labels_batch = horizon_labels.narrow(0, batch_start, batch_size)?;

            // Forward pass
            let predictions = self.model.forward(&seismic_batch)?;

            // Compute loss
            let loss = mse(&predictions, &labels_batch)?;

            // Backward pass
            optimizer.backward_step(&loss)?;

            total_loss += loss.to_scalar::<f32>()?;
            batch_count += 1;
        }

        Ok(total_loss / batch_count as f32)
    }

    /// Get the trained model
    pub fn model(&self) -> &HorizonCNN {
        &self.model
    }

    /// Get training statistics
    pub fn stats(&self) -> &TrainingStats {
        &self.stats
    }

    /// Get the configuration
    pub fn config(&self) -> &TrainingConfig {
        &self.config
    }

    /// Save model weights to file
    pub fn save(&self, path: &str) -> Result<()> {
        self.varmap.save(path)?;
        Ok(())
    }

    /// Load model weights from file
    pub fn load(&mut self, path: &str) -> Result<()> {
        self.varmap.load(path)?;
        Ok(())
    }

    /// Get device from config string
    fn get_device(device_str: &str) -> Result<Device> {
        match device_str.to_lowercase().as_str() {
            "cuda" => Ok(Device::new_cuda(0)?),
            "cpu" => Ok(Device::Cpu),
            _ => Err(candle_core::Error::Msg(format!(
                "Unknown device: {}. Use 'cpu' or 'cuda'",
                device_str
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_config_default() {
        let config = TrainingConfig::default();
        assert_eq!(config.epochs, 100);
        assert_eq!(config.batch_size, 32);
        assert_eq!(config.learning_rate, 1e-3);
    }

    #[test]
    fn test_training_stats_default() {
        let stats = TrainingStats::default();
        assert_eq!(stats.loss_history.len(), 0);
        assert_eq!(stats.best_loss, f32::MAX);
        assert!(!stats.early_stopped);
    }

    #[test]
    fn test_get_device_cpu() {
        let device = Trainer::get_device("cpu").unwrap();
        assert!(matches!(device, Device::Cpu));
    }
}
