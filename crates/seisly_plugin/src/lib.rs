//! StrataForge Plugin System
//!
//! Extensible plugin architecture for custom workflows.

pub mod api;
pub mod manager;
pub mod manifest;

#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "python")]
pub mod interpreter;

pub use api::{Plugin, PluginContext, PluginCommand, PluginError, Result};
pub use manager::PluginManager;
pub use manifest::PluginManifest;
