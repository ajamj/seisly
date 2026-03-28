pub mod json;
pub mod xyz;

use std::path::Path;
use anyhow::Result;
use sf_core::domain::surface::Mesh;

pub trait SurfaceExporter {
    fn export_surface(&self, mesh: &Mesh, path: &Path) -> Result<()>;
}
