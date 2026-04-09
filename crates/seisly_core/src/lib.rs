//! Seisly Core Domain Model
//!
//! This crate contains the core domain types for Seisly,
//! including CRS definitions, entity IDs, and domain entities
//! like wells, trajectories, logs, and surfaces.

pub mod commands;
pub mod crs;
pub mod domain;
pub mod io;
pub mod ipc;
pub mod seismic;
pub mod types;

pub use commands::{Command, UndoRedoStack};
pub use crs::Crs;
pub use domain::{formation_top, log, surface, trajectory, well};
pub use domain::{FormationTop, Log, Surface, Trajectory, Well};
pub use io::{SafeMmap, SafeMmapArc, SafeMmapExt};
pub use seismic::TraceProvider;
pub use types::{DatasetMetadata, EntityId, Provenance};
