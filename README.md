# Seisly

**Rust-Powered Seismic Studio**

[![CI/CD](https://github.com/ajamj/seisly/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/ajamj/seisly/actions/workflows/ci-cd.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-latest-blue)](https://ajamj.github.io/seisly)

**Seisly** (pronounced: /ˈsaɪzli/) is a modern, open-source seismic interpretation platform built in Rust.

> **🎯 Vision:** The fastest, most accessible seismic studio - from exploration to production.

---

## ⚡ Quick Stats

| Metric | Seisly | Petrel | OpendTect |
|--------|--------|--------|-----------|
| **Startup** | <2s | 30-60s | 10-20s |
| **Size** | <100MB | ~5GB | ~2GB |
| **License** | MIT (Free) | $10k+/yr | Open-core |
| **Engine** | Rust + GPU | .NET | C++ |

- **Version:** v0.5.0 (Latest)
- **Platform:** Windows, Linux, macOS
- **Language:** Rust

## 📚 Documentation

- **[📖 Main Documentation](https://ajamj.github.io/StrataForge)** - Complete guides and tutorials
- **[📊 API Reference](https://ajamj.github.io/StrataForge/docs/api/)** - Auto-generated API docs
- **[🚀 Quick Start](QUICKSTART.md)** - Get started in 5 minutes
- **[📋 Blueprint](docs/blueprint.md)** - Strategic roadmap 2026-2028

## ✨ Features

### Core Features (v0.5.0)
- ✅ **Seismic Visualization** - 3D volume rendering with inline/crossline slicing
- ✅ **Horizon Interpretation** - Manual picking, auto-tracking, seed picking
- ✅ **Fault Modeling** - Interactive sketch mode, RBF surface modeling
- ✅ **Velocity Modeling** - Linear velocity model (V0 + kZ)
- ✅ **Time-Depth Conversion** - Real-time depth domain visualization
- ✅ **Synthetic Data Generation** - Generate test data for training/demo **(GUI)** 🎉
- ✅ **Well Data Support** - LAS 2.0/3.0 import, well-seismic tie
- ✅ **Formation Tops** - Stratigraphic marker management
- ✅ **Full SEG-Y Support** - Reader/writer with textual/binary headers
- ✅ **Modern UI** - Light/Dark themes, native menu bar, settings panel ⚙️ 🎉

### Advanced Features
- ✅ **Machine Learning** - U-Net based auto-tracking, CNN fault detection 🤖
- ✅ **GPU Acceleration** - wgpu-based rendering, 10x speedup for attributes ⚡
- ✅ **Seismic Attributes** - 20 attributes (amplitude + frequency) 📊
- ✅ **Quantitative Interpretation** - AVO analysis, FWI, elastic parameters 🔬
- ✅ **4D Monitoring** - Time-lapse seismic, CCUS monitoring 🌊
- ✅ **Plugin System** - Rust + Python plugins 🔌
- ✅ **Production Tools** - Well planning, reservoir surveillance 🛢️

## 📦 Installation

### Prerequisites

**Rust:** Install from https://rustup.rs

**System Dependencies:**

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libxkbcommon-dev \
  libssl-dev \
  pkg-config \
  libgtk-3-dev \
  libfontconfig1-dev \
  protobuf-compiler
```

**Windows:**
```powershell
# Install via Chocolatey
choco install protoc sqlite
```

**macOS:**
```bash
brew install openssl pkg-config
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/ajamj/StrataForge.git
cd StrataForge

# Build release version
cargo build --release

# Run application
cargo run --release --bin sf-app
```

## 🎯 Quick Start

### 1. Launch Application

```bash
cargo run --release --bin sf-app
```

### 2. Generate Synthetic Data (Optional)

For testing without real data:

```rust
use sf_compute::synthetic::*;

// Generate synthetic seismic
let seismic = SyntheticSeismic::new(500, 500, 512);
let data = seismic.generate();

// Generate synthetic well logs
let well = SyntheticWellLog::new("Demo Well", 500000.0, 1000000.0, 50.0, 3000.0);
let (_depths, gr) = well.generate_gr();
```

### 3. Interpret Horizons

1. Select picking mode: **Seed** | **Manual** | **Auto**
2. Click on seismic volume to add picks
3. Horizon mesh auto-generates

### 4. Sketch Faults

1. Select **Sketch Fault** mode
2. Click-drag to draw fault stick
3. Fault surface auto-models with RBF

### 5. Velocity & Depth

1. Set velocity model parameters (V0, k)
2. Toggle **Depth Mode**
3. View data in depth domain

## 📚 Documentation

- **[Quick Start Guide](QUICKSTART.md)** - User tutorial
- **[GitHub Setup](GITHUB_SETUP.md)** - Repository setup
- **[Development Kickoff](DEVELOPMENT_KICKOFF.md)** - Sprint planning
- **[Production Readiness](PRODUCTION_READINESS.md)** - Feature status

## 🏗️ Architecture

```
StrataForge
├── sf_core      - Domain models, types
├── sf_io        - File I/O (SEG-Y, LAS, CSV)
├── sf_compute   - Algorithms (RBF, tracking, volumetrics)
├── sf_storage   - SQLite, blob storage
├── sf_render    - 3D rendering (wgpu)
├── sf_app       - Desktop application (eframe/egui)
├── sf_cli       - Command-line tools
├── sf_ml        - Machine learning (auto-tracking)
├── sf_plugin    - Plugin system
├── sf_attributes - Seismic attribute computation (CPU)
└── sf_attributes_gpu - GPU-accelerated attributes (wgpu)
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run with coverage
cargo tarpaulin --workspace --out Html

# Check code quality
cargo clippy --workspace --all-targets
```

## 🤝 Contributing

We welcome contributions! See our [Development Guide](DEVELOPMENT_KICKOFF.md) for:
- Development workflow
- Sprint planning
- Issue tracking
- Code style guidelines

## 📋 Roadmap

### v0.1.1 (Current) - Beta Release ✅
- Core interpretation features
- Synthetic data generation
- CI/CD pipeline
- Cross-platform builds

### v0.2.0 - Well Integration
- LAS 2.0/3.0 import/export
- Well log visualization
- Well-seismic tie
- Formation tops mapping

### v0.3.0 - Advanced Features
- Auto-tracking enhancement
- Multi-volume blending
- Surface clipping
- Volumetrics export

### v1.0.0 - Production Release
- Complete well workflow
- Performance optimization
- User documentation
- Plugin architecture

## 📞 Support

- **Issues:** https://github.com/ajamj/StrataForge/issues
- **Discussions:** https://github.com/ajamj/StrataForge/discussions
- **Actions:** https://github.com/ajamj/StrataForge/actions

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Built with ❤️ using Rust**

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
