//! StrataForge Compute Module
//!
//! Provides algorithms for triangulation, resampling, and other computations.

pub mod interpolation;
pub mod resampling;
pub mod seismic;
pub mod triangulation;

pub use interpolation::{RbfInterpolator, RbfType};
pub use resampling::resample_trajectory;
pub use triangulation::triangulate_points;
