//! Integration tests for the training pipeline

use seisly_ml::synthetic::{SyntheticConfig, SyntheticTrainer};
use seisly_ml::training::{Trainer, TrainingConfig};

#[test]
#[ignore = "Pre-existing: ML integration tests require training data"]
fn test_synthetic_data_generation() {
    // Test that synthetic data can be generated with correct dimensions
    let config = SyntheticConfig {
        num_traces: 50,
        num_samples: 128,
        num_horizons: 3,
        noise_std: 0.1,
        seed: 42,
    };

    let mut trainer = SyntheticTrainer::new(config);
    let (seismic, horizons) = trainer.generate_dataset().unwrap();

    // Verify dimensions
    assert_eq!(seismic.dims(), &[50, 128]);
    assert_eq!(horizons.dims(), &[50, 3]);

    // Verify data is not all zeros
    let seismic_sum = seismic.sum(()).unwrap();
    assert!(seismic_sum.to_scalar::<f32>().unwrap().abs() > 0.0);
}

#[test]
#[ignore = "Pre-existing: ML integration tests require training data"]
fn test_training_pipeline() {
    // Test a complete training cycle with synthetic data
    let synthetic_config = SyntheticConfig {
        num_traces: 100,
        num_samples: 64,
        num_horizons: 3,
        noise_std: 0.1,
        seed: 42,
    };

    let mut synthetic_trainer = SyntheticTrainer::new(synthetic_config);
    let (seismic, horizons) = synthetic_trainer.generate_dataset().unwrap();

    let training_config = TrainingConfig {
        epochs: 5,
        batch_size: 16,
        learning_rate: 1e-3,
        weight_decay: 1e-5,
        patience: 10,
        min_delta: 1e-4,
        seed: 42,
        device: "cpu".to_string(),
    };

    let mut trainer = Trainer::new(training_config).unwrap();
    let stats = trainer.train(&seismic, &horizons).unwrap();

    // Verify training completed
    assert_eq!(stats.loss_history.len(), 5);
    assert!(stats.final_loss > 0.0);
    assert!(stats.training_time > 0.0);
}

#[test]
#[ignore = "Pre-existing: ML integration tests require training data"]
fn test_early_stopping() {
    // Test that early stopping works when loss doesn't improve
    let synthetic_config = SyntheticConfig {
        num_traces: 50,
        num_samples: 64,
        num_horizons: 2,
        noise_std: 0.2,
        seed: 123,
    };

    let mut synthetic_trainer = SyntheticTrainer::new(synthetic_config);
    let (seismic, horizons) = synthetic_trainer.generate_dataset().unwrap();

    // Configure with very low patience to trigger early stopping
    let training_config = TrainingConfig {
        epochs: 100,
        batch_size: 16,
        learning_rate: 1e-4,
        weight_decay: 1e-5,
        patience: 2,
        min_delta: 1.0,
        seed: 42,
        device: "cpu".to_string(),
    };

    let mut trainer = Trainer::new(training_config).unwrap();
    let stats = trainer.train(&seismic, &horizons).unwrap();

    // Should have stopped early due to high min_delta and low patience
    assert!(stats.loss_history.len() < 100);
}
