//! StrataForge Plugin System
//!
//! Extensible plugin architecture for custom workflows.

pub mod api;
pub mod manager;

#[cfg(feature = "python")]
pub mod python;

pub use api::{Plugin, PluginContext, PluginCommand, PluginError, Result};
pub use manager::PluginManager;

#[cfg(feature = "python")]
pub use python::*;
