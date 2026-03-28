# Roadmap

| Phase | Name | Status | Started | Completed |
|-------|------|--------|---------|-----------|
| v02-phase-b | Seismic Data Slicing & I/O | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v04-phase-a | Structural Foundations (SQLite) | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v04-phase-b | Structural Logic & Interaction | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v04-phase-c | Structural Rendering (3D) | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v05-phase-a | Horizon Interpretation | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v05-phase-b | Velocity & Depth Modeling | ✅ Complete | 2026-03-28 | 2026-03-28 |
| v06 | Advanced Features (ML, Wells) | ⏳ Pending | - | - |
| v1.0 | Production Release | ⏳ Pending | - | - |

## Current Sprint: COMPLETED

**v05-phase-b-velocity-depth** - ✅ Complete

**Accomplishments:**
- Created VelocityPanel UI for velocity model configuration
- Supports Constant and Gradient velocity models
- Real-time velocity preview table
- Depth mode toggle integrated with viewport
- Reused existing LinearVelocityModel for robust conversion

## Next Sprint: v06-advanced-features

**Goal:** Implement advanced features for production use

**Dependencies:** v05-phase-b complete ✅

**Status:** Ready to start

**Potential Features:**
- Well integration (well tops, logs)
- Advanced velocity analysis
- Auto-tracking enhancement
- Export to industry formats

---

## Deferred Items

| Item | Original Phase | Reason | Target Phase |
|------|----------------|--------|--------------|
| Task 2: Transparent Surface Rendering | v04-phase-b | Skipped to prioritize Task 3 | v04-phase-c |

## Key Decisions Log

- **2026-03-28:** Project-per-database architecture (no multi-project single-db)
- **2026-03-28:** PCA-based RBF for 3D fault modeling (supports vertical planes)
- **2026-03-28:** Click-and-drag sketch mode for fault picking (egui-based)
