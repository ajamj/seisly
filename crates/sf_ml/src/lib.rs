//! StrataForge Machine Learning Module
//!
//! CNN-based auto-tracking for horizon interpretation.

pub mod cnn;
pub mod tracker;

pub use cnn::HorizonCNN;
pub use tracker::AutoTracker;
