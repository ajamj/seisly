//! StrataForge Storage Module
//!
//! Provides project format, SQLite storage, and blob store capabilities.

pub mod blob;
pub mod project;
pub mod sqlite;

pub use blob::BlobStore;
pub use project::{Project, ProjectError, ProjectManifest};
