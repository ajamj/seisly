# Requirements

## Phase 1: MVP Foundation (v02 - Completed)
- [x] R1.1: SEG-Y metadata parsing (skeleton)
- [x] R1.2: 3D seismic volume slicing (inline, crossline, timeslice)
- [x] R1.3: Basic seismic viewer with colormaps

## Phase 2: Structural Foundations (v04-a - Completed)
- [x] R2.1: SQLite schema for faults and fault sticks
- [x] R2.2: Persistent storage integration
- [x] R2.3: Project-per-database architecture

## Phase 3: Structural Logic & Interaction (v04-b - Completed)
- [x] R3.1: 3D RBF interpolation for fault surfaces
- [x] R3.2: Interactive fault stick picking (click-and-drag)
- [x] R3.3: Real-time mesh updates
- [x] R3.4: Transparent surface rendering (v04-c)

## Phase 4: Structural Rendering (v04-c - Completed)
- [x] R4.1: 3D fault surface rendering with transparency
- [x] R4.2: Color mapping by displacement/scalar attributes
- [x] R4.3: Fault properties panel (edit name, color, visibility)
- [x] R4.4: Multiple fault layer management
- [x] R4.5: 2D/3D toggle for fault visualization

## Phase 5: Horizon Interpretation (v05-a - Completed)
- [x] R5.1: Horizon picking tools (manual point picking)
- [x] R5.2: Horizon surface generation (Delaunay triangulation)
- [x] R5.3: Multiple horizon management
- [x] R5.4: Horizon property editing (name, type, age)

## Phase 6: Velocity & Depth (v05-b - Completed)
- [x] R6.1: Velocity model building (constant, gradient, layered)
- [x] R6.2: Time-to-depth conversion
- [x] R6.3: Depth domain visualization

## Phase 7: Advanced Features (v06 - Completed)
- [x] R7.1: Auto-fault detection (ML-based)
- [x] R7.2: Well integration (well tops, logs)
- [x] R7.3: Export to industry formats

## Phase 8: Production Ready (v1.0 - Completed)
- [x] R8.1: Performance optimization (GPU acceleration)
- [x] R8.2: User documentation and tutorials
- [x] R8.3: Release packaging and distribution

## Phase 9: Architectural Hardening (v1.1 - Completed)
- [x] HARD-01: High-Performance IPC (Shared Memory for large arrays)
- [x] HARD-02: Worker Resource Hardening (heartbeat watchdog, memory monitoring)
- [x] HARD-03: Data Safety (SIGBUS protection via SafeMmap)
- [x] HARD-04: Interpretive Integrity (Undo/Redo with Command pattern)
- [x] HARD-05: Visual Hardening (Area-weighted normals for smooth shading)
