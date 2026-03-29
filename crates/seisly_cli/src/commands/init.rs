//! Project initialization command

use sf_storage::Project;
use std::path::PathBuf;

pub fn execute(name: String, path: Option<String>, crs: u32) -> anyhow::Result<()> {
    let project_path = path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(format!("{}.sf", name)));

    println!(
        "Creating StrataForge project '{}' at {:?}",
        name, project_path
    );

    let project = Project::create(project_path, name.clone(), format!("EPSG:{}", crs))?;

    println!("✓ Project created successfully");
    println!("  Default CRS: EPSG:{}", crs);
    println!("  Path: {:?}", project.path);
    println!();
    println!("Next steps:");
    println!(
        "  sf import --project {}.sf las --well <name> <file.las>",
        name
    );
    println!(
        "  sf import --project {}.sf trajectory --well <name> <file.csv>",
        name
    );
    println!(
        "  sf import --project {}.sf surface --name <name> <file.xyz>",
        name
    );

    Ok(())
}
