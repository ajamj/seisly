# StrataForge Roadmap

## v0.1 - Offline Wells + Surfaces (Completed)

**Status:** Completed

### Completed
- [x] Workspace skeleton with 8 crates
- [x] Core domain model (Well, Trajectory, Log, Surface)
- [x] CRS support with PROJ
- [x] SQLite schema and blob store
- [x] LAS parser
- [x] Trajectory CSV parser
- [x] XYZ surface parser
- [x] Delaunay triangulation
- [x] Trajectory resampling
- [x] CLI commands (init, import, list)
- [x] Desktop app shell (egui + wgpu)

**Target:** Q2 2026 (Achieved early)

---

## v0.2 - Interactive Seismic & AI Integration (Completed)

**Status:** Completed

### Completed Features
- [x] Desktop App UI
  - [x] Three-panel layout (Explorer, 3D Viewport, Analysis)
  - [x] wgpu integration via PaintCallback
- [x] Seismic I/O & Compute
  - [x] SEG-Y parser scaffold
  - [x] 3D Volume Slicer (Inline/Crossline)
- [x] AI Microservice
  - [x] gRPC Service Contract (ProtoBuf)
  - [x] Python AI Service (PyTorch + gRPC Server)
  - [x] Rust gRPC Client (Tonic)

**Target:** Q3 2026 (Achieved early)

---

## v0.3 - SEG-Y + Slice + Picks (Next)

**Status:** Planned

### Features
- [ ] Real SEG-Y header parsing
- [ ] Memory-mapped volume access
- [ ] Interactive horizon picking
- [ ] Picking mode toggle in UI
- [ ] Surface generation from picks

**Target:** Q4 2026

... (rest of roadmap)
