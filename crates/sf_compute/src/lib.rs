//! StrataForge Compute Module
//! 
//! Provides algorithms for triangulation, resampling, and other computations.

pub mod triangulation;
pub mod resampling;

pub use triangulation::triangulate_points;
pub use resampling::resample_trajectory;
