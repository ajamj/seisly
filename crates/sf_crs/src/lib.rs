//! StrataForge CRS Module
//! 
//! Provides CRS transformation capabilities using PROJ library.

pub mod transformer;
pub mod registry;

pub use transformer::{Transformer, TransformError};
pub use registry::CrsRegistry;
