use std::path::Path;
use anyhow::Result;
use std::io::Write;
use sf_core::domain::surface::Mesh;
use super::SurfaceExporter;

pub struct XyzExporter;

impl SurfaceExporter for XyzExporter {
    fn export_surface(&self, mesh: &Mesh, path: &Path) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        for v in &mesh.vertices {
            writeln!(file, "{} {} {}", v[0], v[1], v[2])?;
        }
        Ok(())
    }
}
