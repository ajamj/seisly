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

---

## v0.2 - Interactive Seismic & AI Integration (Completed)

**Status:** Completed

### Completed Features
- [x] Desktop App UI (Three-panel layout)
- [x] wgpu integration via PaintCallback
- [x] SEG-Y parser scaffold
- [x] 3D Volume Slicer (Inline/Crossline)
- [x] gRPC Bridge between Rust and Python
- [x] Python AI Service scaffold (PyTorch)

---

## v0.3 - SEG-Y + Slice + Picks (Completed)

**Status:** Completed

### Completed Features
- [x] Real SEG-Y header parsing (EBCDIC/Binary)
- [x] High-performance Memory-mapped volume access (`memmap2`)
- [x] Interactive Horizon Picking (Seed & Auto-Track)
- [x] Interpretation Workflow (Explorer & Toolbar)
- [x] Smooth Surface Generation (RBF Interpolation)
- [x] Viewport Visualization (Picks & Smooth Meshes)

---

## v0.4 - Structural Interpretation Module (Completed)

**Status:** Completed

### Completed Features
- [x] Advanced Structural Modeling
  - [x] 3D PCA-based RBF Interpolation
  - [x] Fault Stick sketching mode
- [x] Structural UI (Real-time updates)
- [x] Advanced Rendering (Transparency & Depth sorting)
- [x] Persistence (SQLite storage for Faults)

---

## v0.5 - Depth Conversion & Multi-Volume (Completed)

**Status:** Completed

### Completed Features
- [x] Velocity Modeling Engine
  - [x] Linear Velocity Model ($V_0 + kZ$)
  - [x] Real-time Depth Projection for interpretation data
- [x] Multi-Volume Management
  - [x] Multi-volume registry in project state
  - [x] Multi-volume explorer with channel assignment
  - [x] RGB Blending shader for 2D slices
- [x] Viewport Enhancements
  - [x] Vertical Section View (Inline vs Sample)
  - [x] TWT/Depth vertical axis toggle

---

## v0.6 - Analysis & Export (Completed)

**Status:** Completed

### Completed Features
- [x] Surface Clipping Engine
  - [x] Mesh-surface intersection line calculation
  - [x] Hard cutting (Mesh splitting) along planes
  - [x] Vertical Throw distribution calculation
- [x] Volumetric Engine
  - [x] Grid-based GRV (Gross Rock Volume) integration
  - [x] Multi-selection UI for horizon comparison
- [x] Export Manager
  - [x] XYZ point cloud export for surfaces
  - [x] StrataForge JSON interoperability export
  - [x] Analysis reporting UI

---

## v0.7 - Advanced Visualization & Polish (Next)

**Status:** Planned

### Planned Features
- [ ] Customizable color maps (Viridis, Seismic, etc.)
- [ ] 3D volume rendering (Ray-casting)
- [ ] Undo/Redo interpretation stack
- [ ] Well-log visualization in 3D
- [ ] Final UI/UX polish and icons

**Target:** Q3 2027
