//! Dataset listing command

use sf_storage::Project;
use std::path::PathBuf;

pub fn execute(project_path: String, filter: Option<String>) -> anyhow::Result<()> {
    let project = Project::open(PathBuf::from(&project_path))?;

    println!("StrataForge Project: {}", project.manifest.name);
    println!("Default CRS: {}", project.manifest.default_crs);
    println!("Version: {}", project.manifest.version);
    println!();

    match filter.as_deref() {
        Some("wells") => {
            println!("Wells: (not yet implemented - requires SQLite queries)");
        }
        Some("surfaces") => {
            println!("Surfaces: (not yet implemented - requires SQLite queries)");
        }
        Some("logs") => {
            println!("Logs: (not yet implemented - requires SQLite queries)");
        }
        Some(t) => {
            anyhow::bail!("Unknown type: {}", t);
        }
        None => {
            println!("Datasets: (SQLite queries not yet implemented)");
            println!();
            println!("Project structure:");
            println!("  - metadata.sqlite: (created on first database write)");
            println!("  - blobs/: Content-addressed storage");
            println!("  - cache/: Derived data cache");
        }
    }

    Ok(())
}
