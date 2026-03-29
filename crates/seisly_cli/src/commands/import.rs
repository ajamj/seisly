//! Data import commands

use crate::ImportType;
use sf_io::{LasParser, SurfaceParser, TrajectoryParser};
use sf_storage::{BlobStore, Project};
use std::path::PathBuf;

pub fn execute(project_path: String, import_type: ImportType) -> anyhow::Result<()> {
    let project = Project::open(PathBuf::from(&project_path))?;
    let blob_store = BlobStore::new(project.path.join("blobs"));

    match import_type {
        ImportType::Las { file, well } => {
            println!("Importing LAS file '{}' for well '{}'", file, well);
            let well = LasParser::read(&PathBuf::from(file))?;
            println!("✓ Imported {} logs", well.logs.len());
            for log in &well.logs {
                println!(
                    "  - {}: {} values [{}]",
                    log.mnemonic,
                    log.data.len(),
                    log.units
                );
            }
        }
        ImportType::Trajectory { file, well } => {
            println!("Importing trajectory '{}' from '{}'", well, file);
            let well_id = uuid::Uuid::new_v4(); // In reality, look up well
            let traj = TrajectoryParser::parse(&PathBuf::from(file), well_id)?;
            println!("✓ Imported {} stations", traj.stations.len());
            if let Some(first) = traj.stations.first() {
                println!("  First station: MD={:.2}m", first.md);
            }
            if let Some(last) = traj.stations.last() {
                println!("  Last station: MD={:.2}m", last.md);
            }
        }
        ImportType::Surface { file, name } => {
            println!("Importing surface '{}' from '{}'", name, file);
            let mesh = SurfaceParser::parse(&PathBuf::from(file))?;

            // Serialize mesh to binary
            let mesh_bytes = serialize_mesh(&mesh);
            let hash = blob_store.store(&mesh_bytes)?;

            println!("✓ Imported surface with {} vertices", mesh.vertices.len());
            println!("  Mesh hash: {}", hash);
            println!("  Triangles: {}", mesh.indices.len() / 3);
        }
    }

    Ok(())
}

fn serialize_mesh(mesh: &sf_core::domain::surface::Mesh) -> Vec<u8> {
    // Simple binary serialization
    let mut bytes = Vec::new();

    // Write vertex count
    bytes.extend_from_slice(&(mesh.vertices.len() as u32).to_le_bytes());

    // Write vertices
    for vertex in &mesh.vertices {
        for &coord in vertex {
            bytes.extend_from_slice(&coord.to_le_bytes());
        }
    }

    // Write index count
    bytes.extend_from_slice(&(mesh.indices.len() as u32).to_le_bytes());

    // Write indices
    for &index in &mesh.indices {
        bytes.extend_from_slice(&index.to_le_bytes());
    }

    bytes
}
