use super::SurfaceExporter;
use anyhow::Result;
use sf_core::domain::surface::Mesh;
use std::path::Path;

pub struct JsonExporter;

impl SurfaceExporter for JsonExporter {
    fn export_surface(&self, mesh: &Mesh, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(mesh)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
