# Codebase Structure

**Analysis Date:** 2026-03-30

## Directory Layout

```
myfield/
├── crates/             # Modular Rust crates (seisly_*)
│   ├── seisly_app/     # egui desktop application (GUI)
│   ├── seisly_cli/     # Command line interface
│   ├── seisly_compute/ # Algorithms (triangulation, gridding, etc.)
│   ├── seisly_core/    # Core domain model (Well, Log, Surface, CRS)
│   ├── seisly_crs/     # Coordinate Reference System transformations
│   ├── seisly_io/      # Importers and exporters (LAS, SEG-Y, etc.)
│   ├── seisly_render/  # wgpu rendering primitives
│   ├── seisly_storage/ # SQLite and BLOB storage management
│   └── ...             # Specialized crates (ML, attributes, etc.)
├── docs/               # Technical and architectural documentation
├── scripts/            # Build and utility scripts
├── tests/              # End-to-end and integration tests
├── Cargo.toml          # Workspace configuration
└── pyproject.toml      # Python configuration (for seisly_ai)
```

## Directory Purposes

**crates/seisly_core:**
- Purpose: Foundation of the entire system.
- Contains: Entity IDs, coordinate system types, and core domain entities.
- Key files: `src/lib.rs`, `src/types.rs`, `src/crs.rs`, `src/domain/well.rs`.

**crates/seisly_storage:**
- Purpose: Handles data persistence on disk.
- Contains: Project manifest (`project.yaml`) logic, SQLite schema for metadata, and BLOB store for large binaries.
- Key files: `src/project.rs`, `src/sqlite/mod.rs`, `src/blob/mod.rs`.

**crates/seisly_app:**
- Purpose: Main interactive user interface.
- Contains: egui application logic, viewports, widgets, and app state.
- Key files: `src/app.rs`, `src/main.rs`, `src/ui_styles.rs`.

**crates/seisly_compute:**
- Purpose: Numerical algorithms.
- Contains: Mesh triangulation, spatial interpolation, and seismic processing logic.
- Key files: `src/triangulation.rs`, `src/interpolation.rs`, `src/seismic.rs`.

**crates/seisly_io:**
- Purpose: Data exchange with external formats.
- Contains: LAS, SEG-Y, CSV, and XYZ parsers.
- Key files: `src/las/mod.rs`, `src/segy/mod.rs`.

## Key File Locations

**Entry Points:**
- `crates/seisly_app/src/main.rs`: Desktop GUI entry point.
- `crates/seisly_cli/src/main.rs`: Command-line entry point.

**Configuration:**
- `Cargo.toml`: Workspace definition and shared dependencies.
- `rust-toolchain.toml`: Rust version pinning.

**Core Logic:**
- `crates/seisly_core/src/domain/`: Master definitions of subsurface entities.
- `crates/seisly_storage/src/project.rs`: Logic for project lifecycle (create/open/save).

**Testing:**
- `tests/`: Integration tests covering multi-crate workflows.
- `crates/seisly_*/src/`: Unit tests (co-located in source files).

## Naming Conventions

**Files:**
- `snake_case`: Standard Rust module naming (e.g., `well_manager.rs`).

**Directories:**
- `snake_case`: Module and crate directories.

**Crates:**
- `seisly_*`: All internal workspace crates share this prefix.

## Where to Add New Code

**New Feature (Interpretation):**
- UI: `crates/seisly_app/src/interpretation/`
- Logic: `crates/seisly_core/src/domain/` or specialized crate like `crates/seisly_tracking`.

**New Algorithm:**
- Implementation: `crates/seisly_compute/src/` (create new module if needed).
- Exposed API: Add to `crates/seisly_compute/src/lib.rs`.

**New File Format:**
- Implementation: `crates/seisly_io/src/<format_name>/`.
- Exposed API: Add to `crates/seisly_io/src/lib.rs`.

**New UI Widget:**
- Implementation: `crates/seisly_app/src/widgets/`.

## Special Directories

**target/:**
- Purpose: Compiled artifacts.
- Generated: Yes
- Committed: No

**.planning/:**
- Purpose: GSD project management and codebase mapping.
- Generated: Yes
- Committed: Yes

---

*Structure analysis: 2026-03-30*
