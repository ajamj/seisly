# Tech Stack — Seisly

## Language

- **Primary:** Rust 2021 (stable toolchain, see `rust-toolchain.toml`)
- **Secondary:** Python 3.8+ (for AI/ML plugins, via PyO3 0.22.6)

## Framework & UI

- **UI:** egui 0.29 + eframe 0.29 (immediate mode GUI)
- **Docking:** egui_dock 0.14 with serde (VS Code-style layout)
- **Plotting:** egui_plot 0.29 (well logs, crossplots)
- **GPU Rendering:** wgpu 22.1 (WGSL shaders)

## Database

- **SQLite:** rusqlite 0.31 with bundled feature
- **Blob Store:** Content-addressed via BLAKE3 1.5 hashes

## Build & Packaging

- **Workspace:** Cargo workspace (resolver 2, 20 crates)
- **Python Bindings:** maturin (pyproject.toml, module name: `seisly.seisly`)
- **Releases:** cargo-dist (MSI for Windows, PKG for macOS, deb/tarball for Linux)

## ML & AI

- **Rust ML:** candle-core 0.3, candle-nn 0.3 (HuggingFace)
- **Python ML:** PyTorch + gRPC server (tonic 0.11, prost 0.12)
- **Plugin IPC:** shared_memory 0.12, bytemuck 0.12, JSON-RPC

## Data I/O

- **SEG-Y:** giga-segy-in 0.5, giga-segy-out 0.5
- **LAS:** Custom LAS 2.0/3.0 parser
- **CSV/XYZ:** Custom parsers
- **Memory-Mapped:** memmap2 (wrapped by SafeMmap for bounds safety)

## Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| serde | 1.0 | Serialization (JSON, YAML, bincode) |
| tokio | 1.35 | Async runtime |
| rayon | 1.8 | Parallel iterators |
| ndarray | 0.16 | N-dimensional arrays |
| nalgebra | 0.32 | Linear algebra |
| spade | 2.4 | Delaunay triangulation |
| tracing | 0.1 | Structured logging |
| clap | 4.4 | CLI argument parsing |
| uuid | 1.4 | UUID v4 entity identifiers |
| chrono | 0.4 | Date/time handling |
| proj | 0.31 | Coordinate reference system transforms |

## Testing & Quality

- **Test Runner:** `cargo test --workspace`
- **Benchmarks:** `cargo bench` (criterion)
- **Coverage:** `cargo tarpaulin --fail-under 70` (blocking CI gate)
- **Linter:** `cargo clippy --workspace --all-targets -- -W clippy::all`
- **Formatter:** `cargo fmt --all`
- **CI:** GitHub Actions (fmt → clippy → build → test on Linux + Windows)
