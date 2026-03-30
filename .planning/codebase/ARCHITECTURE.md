# Architecture

**Analysis Date:** 2026-03-30

## Pattern Overview

**Overall:** Crate-based modular workspace architecture with a clear separation of domain logic, infrastructure, and user interface.

**Key Characteristics:**
- **Domain-Centric:** Core entities (`Well`, `Log`, `Surface`) and CRS definitions are isolated in `seisly_core`.
- **Infrastructure-Agnostic Core:** The domain model does not depend on specific storage or UI implementations.
- **Service-Oriented Crates:** Each crate handles a specific responsibility (e.g., `seisly_io` for file parsing, `seisly_compute` for algorithms).
- **GPU-First Rendering:** Visualization is decoupled into `seisly_render` using `wgpu`.

## Layers

**Application Layer:**
- Purpose: Entry points and user interfaces.
- Location: `crates/seisly_app`, `crates/seisly_cli`
- Contains: GUI logic (`egui`), CLI command parsing (`clap`), app-level state management.
- Depends on: `seisly_core`, `seisly_storage`, `seisly_io`, `seisly_compute`, `seisly_render`.
- Used by: End users.

**Visualization Layer:**
- Purpose: Cross-platform GPU rendering primitives.
- Location: `crates/seisly_render`
- Contains: `wgpu` pipelines, mesh/line/point renderers, scene management.
- Depends on: `seisly_core`, `wgpu`, `winit`.
- Used by: `seisly_app`.

**Compute Layer:**
- Purpose: Algorithmic processing and domain logic implementations.
- Location: `crates/seisly_compute`, `crates/seisly_ml`, `crates/seisly_attributes_gpu`
- Contains: Triangulation, gridding, interpolation, ML models, seismic attributes.
- Depends on: `seisly_core`, `ndarray`, `nalgebra`, `candle-core`.
- Used by: `seisly_app`, `seisly_cli`.

**Infrastructure Layer:**
- Purpose: Persistence, I/O, and external library wrappers.
- Location: `crates/seisly_storage`, `crates/seisly_io`, `crates/seisly_crs`
- Contains: SQLite schema, BLOB storage, LAS/SEG-Y parsers, PROJ wrappers.
- Depends on: `seisly_core`, `rusqlite`, `giga-segy`, `proj`.
- Used by: `seisly_app`, `seisly_cli`, `seisly_compute`.

**Domain Layer:**
- Purpose: Fundamental types and business rules.
- Location: `crates/seisly_core`
- Contains: Entity IDs (UUID), CRS definitions, Core domain objects (`Well`, `Log`, `Surface`, `Trajectory`).
- Depends on: `serde`, `uuid`, `chrono`.
- Used by: All other crates.

## Data Flow

**Importing a Dataset (e.g., LAS file):**

1. **Trigger:** `seisly_app` opens a file dialog and captures a file path.
2. **Parsing:** `seisly_io::las::LasParser` reads the file and produces a `seisly_core::domain::Well` and multiple `seisly_core::domain::Log` objects.
3. **Storage:** `seisly_storage::sqlite::SqliteBackend` persists the metadata to `metadata.sqlite`.
4. **Project Update:** `seisly_storage::project::Project` updates the `project.yaml` manifest.
5. **UI Update:** `seisly_app` refreshes its internal state and triggers a re-render in `seisly_render`.

**State Management:**
- **App State:** Managed in `crates/seisly_app/src/app.rs` via the `StrataForgeApp` struct, holding active selections and loaded datasets.
- **Persistence State:** Managed via `crates/seisly_storage/src/project.rs`, which tracks the on-disk state of the project.
- **Undo/Redo:** Handled by `HistoryManager` in `crates/seisly_app/src/interpretation/mod.rs`.

## Key Abstractions

**EntityId:**
- Purpose: Unique identifier (UUID) for every object in the system.
- Examples: `crates/seisly_core/src/types.rs`
- Pattern: Type alias for `uuid::Uuid`.

**Crs (Coordinate Reference System):**
- Purpose: Represents spatial context for all data.
- Examples: `crates/seisly_core/src/crs.rs`
- Pattern: Struct wrapping PROJ strings/EPSG codes with transform logic.

**Project:**
- Purpose: Represents the on-disk container for all data, metadata, and blobs.
- Examples: `crates/seisly_storage/src/project.rs`
- Pattern: Manifest-based project directory structure.

## Entry Points

**Desktop Application:**
- Location: `crates/seisly_app/src/main.rs`
- Triggers: User execution.
- Responsibilities: Initialize `eframe` (egui + wgpu context), load the last project, and start the app loop.

**Command Line Interface:**
- Location: `crates/seisly_cli/src/main.rs`
- Triggers: Shell execution.
- Responsibilities: Execute batch operations (import, export, list) without a GUI.

## Error Handling

**Strategy:** Hierarchical error handling using `thiserror` and `anyhow`.

**Patterns:**
- **Library Errors:** Each crate defines its own `Error` enum using `thiserror` (e.g., `ProjectError` in `seisly_storage`).
- **Application Errors:** `seisly_app` and `seisly_cli` use `anyhow::Result` for top-level error propagation and reporting.

## Cross-Cutting Concerns

**Logging:** Uses `tracing` and `tracing-subscriber` for structured logging. Configured in `seisly_app/src/main.rs`.
**Validation:** Parsers in `seisly_io` perform validation during import; core types use `Result` for construction.
**Authentication:** Currently offline-only. Future `seisly_server` will use token-based auth.

---

*Architecture analysis: 2026-03-30*
