//! StrataForge Machine Learning Module
//!
//! CNN-based auto-tracking for horizon interpretation.

pub mod cnn;
pub mod synthetic;
pub mod tracker;
pub mod training;

pub use cnn::HorizonCNN;
pub use synthetic::{SyntheticConfig, SyntheticTrainer};
pub use tracker::AutoTracker;
pub use training::{TrainingConfig, Trainer};
