//! Deep Learning for Seismic Interpretation
//!
//! U-Net based auto-tracking and fault detection.

pub mod fault_detection;
pub mod training_dl;
pub mod unet;

pub use fault_detection::FaultDetector;
pub use training_dl::DLTrainer;
pub use unet::HorizonUNet;
