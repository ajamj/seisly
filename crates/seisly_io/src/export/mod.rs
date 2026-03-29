pub mod json;
pub mod xyz;

use anyhow::Result;
use sf_core::domain::surface::Mesh;
use std::path::Path;

pub trait SurfaceExporter {
    fn export_surface(&self, mesh: &Mesh, path: &Path) -> Result<()>;
}
