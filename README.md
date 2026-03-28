# StrataForge

**Open-source subsurface interpretation and modeling platform**

StrataForge is a modern, reproducible platform for geoscientists and engineers to analyze subsurface data from exploration to production.

## Features (v0.1 MVP)

- ✅ Project-based offline-first workflow
- ✅ CRS support with PROJ (EPSG codes)
- ✅ LAS well log import
- ✅ Trajectory import from CSV
- ✅ XYZ surface import with triangulation
- ✅ SQLite metadata storage
- ✅ Content-addressed blob store (BLAKE3)

## Quick Start

### Prerequisites

- Rust stable toolchain
- PROJ library (`libproj-dev` on Ubuntu, `proj` on macOS via Homebrew)

On Ubuntu:
```bash
sudo apt-get install libproj-dev proj-bin
```

On macOS:
```bash
brew install proj
```

### Installation

```bash
cargo install --path crates/sf_cli
```

### Create a project

```bash
sf init --name "MyField" --crs 32648
```

### Import data

```bash
# Import well logs
sf import --project MyField.sf las --well "Well-1" well1.las

# Import trajectory
sf import --project MyField.sf trajectory --well "Well-1" traj.csv

# Import surface
sf import --project MyField.sf surface --name "Top1" surface.xyz
```

### List project contents

```bash
sf list --project MyField.sf
```

## Architecture

See [docs/architecture.md](docs/architecture.md) for detailed architecture overview.

## Project Structure

A StrataForge project is a folder with this structure:

```
MyField.sf/
  project.yaml          # Project manifest
  metadata.sqlite       # SQLite database (created on first write)
  blobs/                # Content-addressed blob store
  cache/                # Derived data cache
  workflows/            # Workflow run records
  logs/                 # Application logs
```

## Roadmap

See [docs/roadmap.md](docs/roadmap.md) for development milestones.

### v0.1 - Current
- Offline wells + surfaces
- Basic CLI commands

### v0.2 - Planned
- SEG-Y seismic import
- Seismic slice viewer
- Horizon picking tools

### v0.3 - Planned
- Server mode with REST API
- Remote backend for collaboration

## Development

### Build

```bash
cargo build --workspace
```

### Test

```bash
cargo test --workspace
```

### Lint

```bash
cargo clippy --workspace -- -D warnings
cargo fmt --workspace --check
```

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please read our contributing guidelines before submitting PRs.
