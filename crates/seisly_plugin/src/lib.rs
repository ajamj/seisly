//! Seisly Plugin System
//!
//! Extensible plugin architecture for custom workflows.

pub mod api;
pub mod manager;
pub mod manifest;

#[cfg(feature = "python")]
pub mod bridge;
#[cfg(feature = "python")]
pub mod interpreter;
#[cfg(feature = "python")]
pub mod ipc;
#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "python")]
pub mod python_plugin;

pub use api::{Plugin, PluginCommand, PluginContext, PluginError, Result};
pub use manager::PluginManager;
pub use manifest::PluginManifest;
