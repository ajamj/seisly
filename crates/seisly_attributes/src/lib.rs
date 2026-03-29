//! Seismic Attributes for StrataForge
//!
//! Amplitude and frequency-based attributes for seismic interpretation.

pub mod amplitude;
pub mod frequency;
pub mod trait_def;

pub use amplitude::*;
pub use frequency::*;
pub use trait_def::*;
