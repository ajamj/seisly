# Project Vision

## What We're Building
**StrataForge** - A high-performance 3D seismic interpretation and structural modeling tool for geoscientists.

## Target User
- Geophysicists and geologists in oil & gas exploration
- Seismic interpreters in energy companies
- Research institutions working on subsurface characterization

## Problem Statement
Current seismic interpretation tools are expensive, require proprietary licenses, and lack modern real-time 3D visualization capabilities. StrataForge provides an open-source alternative with interactive fault modeling, horizon picking, and structural analysis.

## Tech Stack
- **Language:** Rust
- **UI Framework:** egui (eframe)
- **Rendering:** wgpu
- **Database:** SQLite (per-project storage)
- **Compute:** ndarray, nalgebra for numerical computing
- **I/O:** segy-rs for seismic data formats

## Key Features
1. **Seismic Data Loading** - SEG-Y format support with inline/crossline slicing
2. **Fault Interpretation** - Interactive fault stick picking with RBF surface modeling
3. **Horizon Picking** - Manual and auto-tracking horizon interpretation
4. **3D Visualization** - Real-time rendering of seismic volumes and structural elements
5. **Velocity Modeling** - Time-depth conversion and velocity analysis
6. **Project Management** - SQLite-based persistent storage

## Architecture Overview
```
┌─────────────────────────────────────────────────────┐
│                    StrataForge App                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │
│  │  sf_app     │  │  sf_render  │  │  sf_compute │  │
│  │  (egui UI)  │  │  (wgpu 3D)  │  │  (RBF, ML)  │  │
│  └─────────────┘  └─────────────┘  └─────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │
│  │  sf_io      │  │  sf_storage │  │  sf_core    │  │
│  │  (SEG-Y)    │  │  (SQLite)   │  │  (Types)    │  │
│  └─────────────┘  └─────────────┘  └─────────────┘  │
└─────────────────────────────────────────────────────┘
```

## Milestones
- **v0.1** - MVP: Seismic loading, fault picking, basic 3D visualization
- **v0.2** - Horizon interpretation, velocity modeling
- **v0.3** - Advanced features: auto-tracking, well integration
- **v1.0** - Production ready: export formats, collaboration features
