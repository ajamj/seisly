# StrataForge Architecture

## Overview

StrataForge is a modern, open-source subsurface interpretation and modeling platform built in Rust. It provides geoscientists and engineers with tools to analyze subsurface data from exploration to production.

## Design Principles

1. **Reproducibility-first:** Every operation is recorded as a deterministic workflow node with parameters and input/output lineage.
2. **Non-destructive editing:** Interpretations and derived outputs are layers; operations create new artifacts rather than overwriting.
3. **Separation of concerns:** Domain model ≠ storage ≠ UI ≠ compute.
4. **GPU-first visualization:** wgpu pipeline for large data visualization.
5. **Open formats & portability:** Projects are folders; heavy blobs are content-addressed.
6. **Offline-first:** Same project format works offline and on server.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      Desktop App (sf_app)                        │
│                    egui + wgpu + winit                           │
├─────────────────────────────────────────────────────────────────┤
│                         CLI (sf_cli)                             │
├─────────────────────────────────────────────────────────────────┤
│                      Backend Trait                               │
│         ┌─────────────────────┬──────────────────────┐          │
│         │    LocalBackend     │    RemoteBackend     │          │
│         │   (SQLite + blobs)  │     (REST API)       │          │
│         └─────────────────────┴──────────────────────┘          │
├─────────────────────────────────────────────────────────────────┤
│  sf_core  │  sf_crs  │  sf_storage  │  sf_io  │  sf_compute    │
├─────────────────────────────────────────────────────────────────┤
│                      Server (sf_server)                          │
│                    axum + REST API                               │
└─────────────────────────────────────────────────────────────────┘
```

## Crate Structure

| Crate | Responsibility |
|-------|----------------|
| `sf_core` | Domain model: IDs, CRS, units, geometry types |
| `sf_crs` | PROJ wrappers, CRS registry, transforms |
| `sf_storage` | Project layout, SQLite schema, blob store |
| `sf_io` | Import/export: LAS, CSV, XYZ, SEG-Y |
| `sf_compute` | Gridding, triangulation, smoothing, resampling |
| `sf_render` | wgpu rendering primitives |
| `sf_cli` | CLI commands: init, import, list |
| `sf_app` | egui desktop application |
| `sf_server` | axum REST server + job runner |

## Project Format

A StrataForge project is a folder:

```
MyField.sf/
  project.yaml          # Manifest with name, CRS, version
  metadata.sqlite       # Structured metadata (wells, logs, surfaces)
  blobs/
    ab/cd/<blake3_hash> # Content-addressed storage
  cache/
    tiles/              # Derived seismic tiles
    decimated/          # LOD meshes
  workflows/
    runs/<uuid>.json    # Workflow execution records
  logs/
    app.log
```

### Content-Addressed Storage

Large binary objects (meshes, seismic tiles) are stored by BLAKE3 hash:
- Deduplication automatic
- Integrity verification on read
- Location: `blobs/<hash[0:2]>/<hash[2:4]>/<hash>`

## Data Model

### Core Entities

All entities share common metadata:
- `id: Uuid` - Unique identifier
- `name: String` - Human-readable name
- `crs: Crs` - Coordinate reference system
- `created_at: DateTime` - Creation timestamp
- `provenance: Option<Provenance>` - Lineage tracking

### Wells
```
Well
├── metadata (id, name, crs, ...)
├── head_x, head_y (project CRS)
└── kb_elevation (optional)

Trajectory
├── id
├── well_id
└── stations: Vec<Station>
    └── { md, x, y, z }

Log
├── id
├── well_id
├── depth_mnemonic (MD/TVD)
└── curves: Vec<Curve>
    └── { mnemonic, unit, values }
```

### Surfaces
```
Surface
├── metadata
└── mesh_ref: BlobRef
    └── { hash, size_bytes }

Mesh
├── vertices: Vec<[f32; 3]>
├── indices: Vec<u32>
└── normals: Option<Vec<[f32; 3]>>
```

## CRS Handling

- Uses PROJ library via `proj` crate
- Every dataset has its own CRS
- Project has default CRS
- Transforms recorded with provenance
- EPSG codes preferred (e.g., "EPSG:32648")

## Storage Layer

### SQLite Schema

Tables:
- `datasets` - Master dataset registry
- `wells`, `trajectories`, `logs`, `curves`
- `surfaces`
- `seismic_volumes` (v0.2)
- `picks` (v0.2)
- `workflow_runs`

### Blob Store API

```rust
let store = BlobStore::new(project.blobs_path());
let hash = store.store(&content)?;  // Returns BLAKE3 hash
let content = store.retrieve(&hash)?; // Verifies hash
```

## Compute Layer

### Algorithms

- **Triangulation:** Delaunay via `spade` crate
- **Resampling:** Linear interpolation for trajectories
- **Gridding:** (planned) Kriging, IDW
- **Seismic attributes:** (planned)

### Job Model

```rust
// Offline mode: in-process
let result = triangulate_points(&points)?;

// Server mode: async job
let job_id = server.submit_job(Triangulate { points })?;
let result = server.poll_job(job_id)?;
```

## Rendering (v0.1 stub)

- **wgpu** for cross-platform GPU rendering
- **MeshRenderer** for surfaces
- **LineRenderer** for trajectories, fault sticks
- **Scene** manages renderable objects

Full egui integration planned for v0.2.

## Extension Points

### Backend Trait

```rust
trait Backend {
    async fn list_datasets(&self) -> Result<Vec<Dataset>>;
    async fn get_dataset(&self, id: Uuid) -> Result<Dataset>;
    async fn add_pick(&self, pick: Pick) -> Result<()>;
    async fn run_job(&self, job: Job) -> Result<JobId>;
}
```

Implementations:
- `LocalBackend` - Direct SQLite + blob access
- `RemoteBackend` - REST API client

### IO Parsers

```rust
trait Parser<T> {
    fn parse(path: &Path) -> Result<T>;
}
```

Implementations:
- `LasParser` - LAS 2.0/3.0
- `TrajectoryParser` - CSV
- `SurfaceParser` - XYZ
- `SegyParser` - (planned)

## Security Considerations

### v0.1 (offline)
- Local file access only
- No network exposure
- Input validation on parsers

### Future (server mode)
- Token-based auth (planned)
- OIDC integration (planned)
- Rate limiting on API
- Project-level access control

## Performance Targets

| Operation | Target |
|-----------|--------|
| Open project | < 1s |
| Load well log (1000 points) | < 100ms |
| Triangulate surface (10k points) | < 2s |
| Render 100k triangles | 60 FPS |
| Seismic slice extraction | < 500ms |

## Testing Strategy

- **Unit tests:** Domain logic, parsers, transforms
- **Integration tests:** Full import → query → export workflows
- **Snapshot tests:** Mesh outputs, tile hashes (planned)

Run all tests:
```bash
cargo test --workspace
```

## See Also

- [Project Format](project_format.md)
- [Roadmap](roadmap.md)
- [API Documentation](api.md) (planned)
