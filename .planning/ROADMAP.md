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

## Current Phase: v1.2 (Next)

**Goal:** [To be planned]

**Requirements:** [To be defined]

**Plans:** [To be planned]

---

## Completed Phases

**v1.1-hardening** - ✅ Complete (2026-04-01)
- High-performance Shared Memory IPC for large seismic arrays
- Worker resource hardening with heartbeat watchdog
- SIGBUS protection via SafeMmap wrapper
- Undo/Redo infrastructure for interpretation operations
- Area-weighted normal computation for smooth shading
- All 5 HARD requirements satisfied (100/100 health score)

**v1.0-production-release** - ✅ Complete

---

## Completed Phases

**v1.0-production-release** - ✅ Complete
- Branding, Docs & Quality Audit.
- Performance & Robustness improvements.
- Secure Plugin Architecture (isolated Python workers).
- Professional UI/UX with docking support.
- Distribution via `cargo-dist`.

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

- **2026-04-01:** Implement Shared Memory IPC for large seismic arrays (v1.1-01).
- **2026-04-01:** Adopt `memmap2` SIGBUS protection (v1.1-03).
- **2026-03-31:** Switch to process-isolated Python worker model for plugin security (v1.0-03).
- **2026-03-31:** Adopt `cargo-dist` for multi-platform distribution (v1.0-05).
- **2026-03-28:** Project-per-database architecture (no multi-project single-db)
- **2026-03-28:** PCA-based RBF for 3D fault modeling
- **2026-03-28:** Click-and-drag sketch mode for fault picking
