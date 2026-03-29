//! Deep Learning for Seismic Interpretation
//!
//! U-Net based auto-tracking and fault detection.

pub mod unet;
pub mod training_dl;
pub mod fault_detection;

pub use unet::HorizonUNet;
pub use training_dl::DLTrainer;
pub use fault_detection::FaultDetector;
