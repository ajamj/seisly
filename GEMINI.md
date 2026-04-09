# QWEN.md — Seisly Project Context

This file provides instructional context for AI agents (Qwen Code, Claude Code, etc.) when working with this repository.

---

## 🎯 Project Overview

**Seisly** (pronounced /ˈsaɪzli/) is a professional open-source seismic interpretation platform built in Rust. It enables geoscientists to load, visualize, and interpret 3D seismic data for oil & gas exploration, geothermal, and CCUS applications.

**Version:** 1.0.0 (production-grade)  
**License:** MIT OR Apache-2.0  
**Repository:** https://github.com/ajamj/seisly

### Key Capabilities
- **SEG-Y I/O** — High-performance memory-mapped SEG-Y import/export with persistent sidecar indexing
- **3D Visualization** — wgpu-based rendering with colormaps, wiggle traces, and interactive slicing
- **Horizon Interpretation** — Manual picking, BFS auto-tracking, seed point expansion
- **Fault Modeling** — Interactive sketch mode with RBF surface generation
- **Well Integration** — LAS 2.0/3.0 support, formation tops, well-seismic tie
- **Advanced Attributes** — 20+ CPU and GPU-accelerated seismic attributes
- **Python Extensibility** — Process-isolated Python worker for safe AI/ML plugin execution

---

## 🏗️ Architecture

### Workspace Structure

Seisly is a Cargo workspace with **20 crates** organized in a layered architecture:

| Layer | Crates | Purpose |
|-------|--------|---------|
| **Core** | `seisly_core`, `seisly_crs`, `seisly_storage` | Domain types, CRS, SQLite database, blob store |
| **I/O** | `seisly_io` | SEG-Y, LAS 2.0/3.0, CSV, XYZ file I/O |
| **Compute** | `seisly_compute`, `seisly_attributes`, `seisly_attributes_gpu` | Algorithms: RBF, triangulation, seismic attributes |
| **Rendering** | `seisly_render` | wgpu-based 3D rendering engine |
| **Application** | `seisly_app`, `seisly_cli` | Desktop GUI (egui) and CLI (`sf` binary) |
| **ML/AI** | `seisly_ml`, `seisly_ml_deep`, `seisly_ai`, `seisly_py_worker` | CNN tracking, U-Net fault detection, gRPC AI server |
| **Advanced** | `seisly_qi`, `seisly_4d`, `seisly_tracking`, `seisly_fwi`, `seisly_production` | AVO analysis, time-lapse, FWI, well planning |
| **Plugin** | `seisly_plugin` | Rust + Python plugin manager |

### Data Flow

```
SEG-Y File → MmappedSegy (mmap) → SeislyIndex (sidecar cache) → TraceProvider
                                                                    ↓
SeismicVolume (slice extraction: inline/crossline/time) → SeismicRenderer (wgpu)
                                                                    ↓
ViewportWidget (egui) → egui_wgpu::Callback → GPU Texture → Display
```

### Key Design Patterns
- **Trait-based abstraction:** `TraceProvider` enables mock providers for testing
- **ECS-style resources:** `egui_wgpu::CallbackResources` for GPU state management
- **Command pattern:** `HistoryManager` for undo/redo (100 steps)
- **Background threading:** `mpsc::channel` for async operations (SEG-Y scanning)
- **Content-addressed storage:** BLAKE3-hashed blobs in project `blobs/` directory

---

## 🛠️ Building and Running

### Prerequisites
- **Rust:** Stable toolchain (see `rust-toolchain.toml`) — `rustup` recommended
- **System deps:**
  - **Windows:** `choco install protoc sqlite`
  - **macOS:** `brew install openssl pkg-config proj`
  - **Linux (Ubuntu/Debian):** `sudo apt-get install -y libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev pkg-config libgtk-3-dev libfontconfig1-dev protobuf-compiler libproj-dev proj-bin`

### Build Commands

```bash
# Build entire workspace
cargo build --workspace

# Build release version
cargo build --release --workspace

# Build specific crate
cargo build -p seisly_cli

# Run desktop application
cargo run --release --bin seisly-app

# Run with Python plugin support
cargo run --release --bin seisly-app --features python

# Run CLI tool
cargo run --release --bin sf -- <args>
```

### Test Commands

```bash
# Run all tests
cargo test --workspace

# Run tests with output (don't stop on first failure)
cargo test --workspace --verbose --no-fail-fast

# Run specific crate tests
cargo test -p seisly_core

# Run benchmarks
cargo bench --workspace

# Code coverage (requires cargo-llvm-cov)
cargo llvm-cov --workspace --html
```

### Lint Commands

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all

# Run Clippy (recommended: -W clippy::all)
cargo clippy --workspace --all-targets -- -W clippy::all

# Fix auto-fixable Clippy warnings
cargo clippy --workspace --all-targets --fix
```

### Coverage Thresholds

Enforced by `.coverage-thresholds.json` with `blocking_gate: true`:

| Crate | Line | Branch |
|-------|------|--------|
| `seisly_io` | 80% | 65% |
| `seisly_render` | 75% | 60% |
| `seisly_core` | 70% | 55% |
| `seisly_compute` | 70% | 55% |
| `seisly_app` | 65% | 45% |
| `seisly_storage` | 60% | 40% |

---

## 📋 Development Conventions

### Code Style
- **Rust Edition:** 2021
- **Formatting:** `cargo fmt` — 4-space indentation, standard Rust style
- **Linting:** `cargo clippy -- -W clippy::all` — all warnings should be fixed
- **Error handling:** `thiserror` for library errors, `anyhow` for application errors
- **IDs:** UUID v4 for all entity identifiers

### Testing Practices
- Tests are **inline** in source files (`#[cfg(test)]` modules), not in separate `tests/` directories
- Use `InMemoryProvider` for mocking seismic data in tests
- Synthetic SEG-Y buffers generated via test fixtures (see `seisly_io` tests)
- NaN safety tested explicitly for float comparison operations
- GPU shader compilation verified via pipeline init tests

### Git Workflow
- **Branch naming:** Feature branches for implementation work
- **Commits:** Atomic per task — each commit should be independently testable
- **CI/CD:** `.github/workflows/ci-cd.yml` runs fmt → clippy → build → test on Linux + Windows

### Project Format
Seisly projects are directories with `.sf` extension:
```
MyField.sf/
  project.yaml          # Manifest: name, CRS, version
  metadata.sqlite       # Structured metadata (SQLite)
  blobs/                # Content-addressed storage (BLAKE3 hashes)
  cache/                # Derived data cache
  workflows/            # Workflow execution records
  logs/                 # Application logs
```

---

## 🤖 Custom Agent Definitions

The project defines 8 specialized agents in `agents/` for the GSD (Get Shit Done) workflow:

| Agent | File | Purpose |
|-------|------|---------|
| Product Manager | `agents/product-manager.md` | Validate use cases, user benefit, scope |
| Architect | `agents/architect.md` | Review technical architecture, dependencies |
| Designer | `agents/designer.md` | Review UX, API design, developer experience |
| Security Design | `agents/security-design.md` | Review security vulnerabilities, threat modeling |
| CTO | `agents/cto.md` | Review TDD readiness, strategic viability |
| Planner | `agents/planner.md` | Create executable phase plans (PLAN.md files) |
| Phase Researcher | `agents/phase-researcher.md` | Research technical approaches for a phase |
| Plan Checker | `agents/plan-checker.md` | Verify plan quality before execution |

These agents are used by the design review gate and planning workflows. The `design-review-gate` skill auto-detects these files and uses them as agent prompts (falling back to inline prompts if not found).

---

## 📚 Documentation

- **User Manual:** `docs/user-manual/SUMMARY.md`
- **API Reference:** Generated at `api/` (Jekyll site)
- **Planning:** `.planning/` directory contains STATE.md, ROADMAP.md, phase plans, and codebase maps
- **Codebase Maps:** `.planning/codebase/*.md` — 7 structured documents covering stack, architecture, conventions, testing, concerns, integrations, and structure

---

## 🔑 Key Files to Read First

When starting work on a new feature, read these files in order:

1. **`CLAUDE.md`** — Build commands, test commands, system dependencies
2. **`Cargo.toml`** — Workspace structure, dependency versions
3. **`.coverage-thresholds.json`** — Coverage requirements per crate
4. **Crate-specific source** — Navigate to the relevant crate's `src/` directory

For rendering work:
1. `crates/seisly_render/src/shaders/seismic.wgsl` — Shader contract
2. `crates/seisly_render/src/seismic_renderer.rs` — Renderer implementation
3. `crates/seisly_app/src/widgets/viewport.rs` — UI ↔ rendering bridge

For I/O work:
1. `crates/seisly_io/src/segy/mmap.rs` — Memory-mapped SEG-Y access
2. `crates/seisly_io/src/segy/index.rs` — Sidecar indexing
3. `crates/seisly_io/src/segy/parser.rs` — Header parsing

---

## ⚠️ Important Notes

- **Do NOT translate technical terms** in code or documentation (SEG-Y, LAS, CRS, RBF, etc.)
- **SEG-Y byte positions:** Inline = bytes 188-191 (big-endian), Crossline = bytes 192-195 (big-endian)
- **SafeMmap:** `as_slice()` is `unsafe` — prefer `get()` and `get_slice()` for bounded access
- **Float comparisons:** Use `total_cmp()` instead of `partial_cmp().unwrap()` to avoid NaN panics
- **Plugin security:** Python plugins run in process-isolated workers — never trust plugin output without validation
- **OpenTect reference:** Code in `references/OpendTect/` is for **algorithmic understanding only** — do NOT copy implementation (GPL licensing risk)
