//! StrataForge Compute Module
//!
//! Provides algorithms for triangulation, resampling, and other computations.

pub mod clipping;
pub mod interpolation;
pub mod resampling;
pub mod seismic;
pub mod synthetic;
pub mod throw;
pub mod tracking;
pub mod triangulation;
pub mod velocity;
pub mod volumetrics;
pub mod well_tie;

pub use clipping::{intersect_mesh_plane, update_surface_intersections, Plane};
pub use interpolation::{RbfInterpolator, RbfType};
pub use resampling::{interpolate_station, resample_trajectory};
pub use synthetic::{SyntheticHorizonPicks, SyntheticSeismic, SyntheticWellLog};
pub use throw::calculate_throw_distribution;
pub use tracking::{snap_to_extrema, track_event};
pub use triangulation::triangulate_points;
pub use velocity::LinearVelocityModel;
pub use well_tie::{WellTieEngine, WellTie, TimeDepthPair, TieParameters};
