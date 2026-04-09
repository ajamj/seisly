# Roadmap

| Phase | Name | Status | Started | Completed |
|-------|------|--------|---------|-----------|
| v02-phase-b | Seismic Data Slicing & I/O | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v04-phase-a | Structural Foundations (SQLite) | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v04-phase-b | Structural Logic & Interaction | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v04-phase-c | Structural Rendering (3D) | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v05-phase-a | Horizon Interpretation | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v05-phase-b | Velocity & Depth Modeling | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v06 | Advanced Features (ML, Wells) | ✅ Complete | 2026-03-30 | 2026-03-31 |
| v1.0 | Production Release | ✅ Complete | 2026-03-31 | 2026-03-31 |
| v1.1 | Architectural Hardening | ✅ Complete | 2026-04-01 | 2026-04-01 |
| v1.2 | Compilation & Phase 2 Integration | ✅ Complete | 2026-04-08 | 2026-04-08 |
| v1.3 | Seismic Visualization & Plotting | 🏗️ Planning | 2026-04-08 | |

## Current Phase: v1.3

**Goal:** Implement a professional seismic plotting engine with real-time 2D slicing, variable intensity color mapping, and wiggle trace overlays.

**Requirements:**
- **v1.3-visualization:** Real-time extraction and rendering of seismic slices (Inlines, Crosslines, Time Slices).
- **v1.3-color-mapping:** Support for standard seismic colormaps (Blue-White-Red, Gray).
- **v1.3-wiggle-plotting:** Optional wiggle-trace overlay for detailed structural interpretation.
- **v1.3-performance:** Implement persistent sidecar indexing and O(1) grid mapping to eliminate SEG-Y import lag.

**Plans:**
- [v1.3-seismic-visualization](./phases/v1.3-seismic-visualization-PLAN.md)

---

## Completed Phases

**v1.2-compilation** - ✅ Complete (2026-04-08)
- Automated release build for x86_64-pc-windows-gnu.
- Integration of Phase 2 features (QI, 4D, GPU Acceleration).
- Async GPU initialization with shared-device state.

**v1.1-hardening** - ✅ Complete (2026-04-01)
- High-performance Shared Memory IPC for large seismic arrays.
- SIGBUS protection via SafeMmap wrapper.
- Undo/Redo infrastructure for interpretation operations.

... (rest of history remains)
