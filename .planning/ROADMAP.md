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

## Current Phase: RELEASED

**Goal:** Post-launch monitoring and v1.1 planning.

**Requirements:** R8.1, R8.2, R8.3, SEC-01, UI-01, ERR-01

**Plans:** 5 plans
- [x] v1.0-01-PLAN.md — Branding, Docs & Quality Audit ✅ (2026-03-31)
- [x] v1.0-02-PLAN.md — Performance & Robustness ✅ (2026-03-31)
- [x] v1.0-03-PLAN.md — Secure Plugin Architecture ✅ (2026-03-31)
- [x] v1.0-04-PLAN.md — Professional UI/UX ✅ (2026-03-31)
- [x] v1.0-05-PLAN.md — Distribution & Release ✅ (2026-03-31)

---

## Completed Phases

**v06-advanced-features** - ✅ Complete
- Integrated Plugin Manager UI.
- High-performance zero-copy data bridge to NumPy.
- PythonPlugin implementation with dynamic sys.path management.
- Embedded Python interpreter using PyO3.

---

## Deferred Items

| Item | Original Phase | Reason | Target Phase |
|------|----------------|--------|--------------|
| (None) | - | - | - |

## Key Decisions Log

- **2026-03-31:** Switch to process-isolated Python worker model for plugin security (v1.0-03).
- **2026-03-31:** Adopt `cargo-dist` for multi-platform distribution (v1.0-05).
- **2026-03-28:** Project-per-database architecture (no multi-project single-db)
- **2026-03-28:** PCA-based RBF for 3D fault modeling
- **2026-03-28:** Click-and-drag sketch mode for fault picking
