use std::path::Path;
use anyhow::Result;
use sf_core::domain::surface::Mesh;
use super::SurfaceExporter;

pub struct JsonExporter;

impl SurfaceExporter for JsonExporter {
    fn export_surface(&self, mesh: &Mesh, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(mesh)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
