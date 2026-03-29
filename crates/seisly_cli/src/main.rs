//! StrataForge CLI
//!
//! Command-line interface for StrataForge subsurface data management.

mod commands;

use clap::{Parser, Subcommand};
use commands::{import, init, list};

#[derive(Parser)]
#[command(name = "sf")]
#[command(about = "StrataForge CLI - Subsurface Data Management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new StrataForge project
    Init {
        /// Project name
        #[arg(short, long)]
        name: String,

        /// Project path
        #[arg(short, long)]
        path: Option<String>,

        /// Default CRS (EPSG code)
        #[arg(long, default_value = "32648")]
        crs: u32,
    },

    /// Import data into a project
    Import {
        /// Project path
        #[arg(short, long)]
        project: String,

        #[command(subcommand)]
        import_type: ImportType,
    },

    /// List datasets in a project
    List {
        /// Project path
        #[arg(short, long)]
        project: String,

        /// Filter by type (wells, logs, surfaces)
        #[arg(short, long)]
        r#type: Option<String>,
    },
}

#[derive(Subcommand)]
enum ImportType {
    /// Import LAS well log file
    Las {
        /// LAS file path
        file: String,

        /// Well name
        #[arg(short, long)]
        well: String,
    },

    /// Import trajectory CSV
    Trajectory {
        /// CSV file path
        file: String,

        /// Well name or ID
        #[arg(short, long)]
        well: String,
    },

    /// Import XYZ surface
    Surface {
        /// XYZ file path
        file: String,

        /// Surface name
        #[arg(short, long)]
        name: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, path, crs } => {
            init::execute(name, path, crs)?;
        }
        Commands::Import {
            project,
            import_type,
        } => {
            import::execute(project, import_type)?;
        }
        Commands::List { project, r#type } => {
            list::execute(project, r#type)?;
        }
    }

    Ok(())
}
