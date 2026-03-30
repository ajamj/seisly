# Technology Stack

**Analysis Date:** 2025-03-28

## Languages

**Primary:**
- Rust (1.70+) - Core platform, GUI, I/O, and computation engine. (`Cargo.toml`, `crates/`)

**Secondary:**
- Python (3.8+) - AI/ML backend services and Python bindings. (`crates/seisly_ai/`, `python/`, `pyproject.toml`)
- SQL - Metadata persistence via SQLite. (`crates/seisly_storage/`)

## Runtime

**Environment:**
- Rust compiled binaries (Native performance)
- Python runtime for AI server (`crates/seisly_ai/server.py`)

**Package Manager:**
- Cargo (Rust)
- pip (Python)
- Lockfile: `Cargo.lock` present

## Frameworks

**Core:**
- `tokio` (1.35) - Asynchronous runtime. (`Cargo.toml`)
- `axum` (0.7) - REST API framework. (`Cargo.toml`)
- `tonic` (0.11) - gRPC framework for cross-language communication. (`Cargo.toml`, `crates/seisly_app/Cargo.toml`)

**GUI:**
- `egui` / `eframe` (0.26) - Primary desktop UI framework. (`crates/seisly_app/Cargo.toml`)
- `wgpu` (0.19) - WebGPU-based hardware-accelerated rendering. (`crates/seisly_render/Cargo.toml`)

**Testing:**
- `cargo test` - Integrated testing.
- `tempfile` (3.9) - Temporary file management for tests.
- `tarpaulin` - Test coverage reporting (referenced in `README.md`).

**Build/Dev:**
- `maturin` - Build backend for Python bindings. (`pyproject.toml`)
- `prost` / `tonic-build` - Protocol Buffer and gRPC code generation. (`crates/seisly_app/Cargo.toml`)

## Key Dependencies

**Critical:**
- `serde` (1.0) - Serialization/deserialization framework. (`Cargo.toml`)
- `rusqlite` (0.31) - SQLite database driver. (`crates/seisly_storage/Cargo.toml`)
- `giga-segy-in/out` (0.5) - SEG-Y seismic format support. (`crates/seisly_io/Cargo.toml`)
- `candle-core` (0.3) - Rust-native machine learning framework. (`Cargo.toml`)
- `torch` - PyTorch for the Python-based AI server. (`crates/seisly_ai/requirements.txt`)

**Infrastructure:**
- `rayon` (1.8) - Parallel data processing. (`Cargo.toml`)
- `nalgebra` (0.32) - Linear algebra for rendering and compute. (`Cargo.toml`)
- `ndarray` (0.15) - N-dimensional arrays for seismic data handling. (`Cargo.toml`)
- `proj` (0.31) - Coordinate Reference System (CRS) transformations. (`Cargo.toml`)

## Configuration

**Environment:**
- Configured via `project.yaml` within project folders.
- `metadata.sqlite` for project state.

**Build:**
- `Cargo.toml` (Workspace and crate level)
- `pyproject.toml` (Python bindings)
- `rust-toolchain.toml` (Rust version)

## Platform Requirements

**Development:**
- Rust stable toolchain
- PROJ library (`libproj-dev` or `proj`)
- Protobuf compiler (`protoc`)
- SQLite

**Production:**
- Windows, Linux, macOS (Native binaries)

---

*Stack analysis: 2025-03-28*
