//! StrataForge CRS Module
//!
//! Provides CRS transformation capabilities using PROJ library.

pub mod registry;
pub mod transformer;

pub use registry::CrsRegistry;
pub use transformer::{TransformError, Transformer};
