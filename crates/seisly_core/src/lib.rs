//! StrataForge Core Domain Model
//!
//! This crate contains the core domain types for StrataForge,
//! including CRS definitions, entity IDs, and domain entities
//! like wells, trajectories, logs, and surfaces.

pub mod crs;
pub mod domain;
pub mod types;

pub use crs::Crs;
pub use domain::{formation_top, log, surface, trajectory, well};
pub use domain::{FormationTop, Log, Surface, Trajectory, Well};
pub use types::{DatasetMetadata, EntityId, Provenance};
