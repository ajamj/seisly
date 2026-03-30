# External Integrations

**Analysis Date:** 2025-03-28

## APIs & External Services

**Geospatial:**
- PROJ - Used for Coordinate Reference System (CRS) transformations and projections. (`crates/seisly_crs/Cargo.toml`, `Cargo.toml`)
  - Client: `proj` crate.

**Communication:**
- gRPC - Used for communication between the Rust frontend/core and Python AI services. (`crates/seisly_ai/server.py`, `crates/seisly_app/Cargo.toml`)
  - Implementation: `tonic` (Rust), `grpcio` (Python).

## Data Storage

**Databases:**
- SQLite (Embedded) - Metadata, project state, and interpretation management. (`crates/seisly_storage/`)
  - Connection: `metadata.sqlite` in project directories.
  - Client: `rusqlite` crate.

**File Storage:**
- Content-Addressed Blob Storage - Used for storing large seismic volumes and well logs. (`crates/seisly_storage/`)
  - Implementation: `blobs/` subdirectory in project structure.
  - Hashing: `blake3` crate.

**Seismic Data:**
- SEG-Y - Standard industry format for seismic volumes.
  - Client: `giga-segy-in` / `giga-segy-out` crates. (`crates/seisly_io/src/segy/`)

**Well Log Data:**
- LAS (2.0/3.0) - Log ASCII Standard for well data.
  - Implementation: Custom parser in `crates/seisly_io/src/las/`.

## Authentication & Identity

**Auth Provider:**
- Custom / Local only (Currently no centralized auth).

## Monitoring & Observability

**Error Tracking:**
- None detected.

**Logs:**
- `tracing` / `tracing-subscriber` - Integrated logging for both async and sync components. (`Cargo.toml`)

## CI/CD & Deployment

**Hosting:**
- Local Desktop deployment.

**CI Pipeline:**
- GitHub Actions - Automated testing, linting, and release management. (`.github/workflows/ci-cd.yml`)

## Environment Configuration

**Required env vars:**
- `RUST_LOG` - Configures log verbosity via `tracing-subscriber`.

**Secrets location:**
- Not applicable (No cloud secrets identified in core).

## Webhooks & Callbacks

**Incoming:**
- None detected.

**Outgoing:**
- None detected.

---

*Integration audit: 2025-03-28*
