//! StrataForge Compute Module
//!
//! Provides algorithms for triangulation, resampling, and other computations.

pub mod interpolation;
pub mod resampling;
pub mod seismic;
pub mod tracking;
pub mod triangulation;

pub use interpolation::{RbfInterpolator, RbfType};
pub use resampling::resample_trajectory;
pub use tracking::{snap_to_extrema, track_event};
pub use triangulation::triangulate_points;
