//! StrataForge Storage Module
//! 
//! Provides project format, SQLite storage, and blob store capabilities.

pub mod project;
pub mod sqlite;
pub mod blob;

pub use project::{Project, ProjectManifest, ProjectError};
pub use blob::BlobStore;
