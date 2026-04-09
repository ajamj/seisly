//! Enhanced Auto-Tracking
//!
//! Multi-horizon tracking, fault-guided tracking, and quality control.

pub mod fault_guided;
pub mod multi_horizon;
pub mod quality;

pub use fault_guided::FaultGuidedTracker;
pub use multi_horizon::MultiHorizonTracker;
pub use quality::TrackingQuality;
