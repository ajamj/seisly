//! StrataForge Core Domain Model
//! 
//! This crate contains the core domain types for StrataForge,
//! including CRS definitions, entity IDs, and domain entities
//! like wells, trajectories, logs, and surfaces.

pub mod crs;
pub mod types;
pub mod domain;

pub use crs::Crs;
pub use types::{DatasetMetadata, EntityId, Provenance};
pub use domain::{well, trajectory, log, surface};
