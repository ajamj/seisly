---
phase: v04-phase-b-structural-logic
plan: 2026-03-28-v04-phase-b-structural-logic.md
subsystem: seisly_app
tags: [RBF, 3D, Fault Modeling, Interactive Picking, Sketch Mode]
dependency_graph:
  requires: [seisly_compute, seisly_render, egui]
  provides: [Interactive Fault Picking]
  affects: [seisly_app]
tech_stack:
  added: []
  patterns: [Click-and-Drag Sketching for Fault Sticks]
key_files:
  - crates/seisly_compute/src/interpolation.rs
  - crates/seisly_app/src/interpretation/mod.rs
  - crates/seisly_app/src/widgets/viewport.rs
  - crates/seisly_app/src/app.rs
decisions:
  - Used PCA (SVD) to find the best-fitting plane of 3D point clouds.
  - Performed RBF interpolation in the local (u, v, n) coordinate system to support vertical and high-angle planes.
  - Implemented `SketchFault` mode using `egui` drag events to allow free-form fault stick drawing.
  - Automated RBF surface updates upon completing a fault stick sketch.
metrics:
  duration: 45m
  completed_date: "2026-03-28"
  tasks_total: 3
  tasks_completed: 2
---

# Phase v04 Plan B: Structural Logic & Visuals Summary

Implemented 3D RBF interpolation and interactive fault picking.

## Task 1: 3D RBF Interpolation
- **Goal:** Adapt `RbfInterpolator` for 3D inputs to handle high-angle/vertical planes.
- **Outcome:** Successfully implemented PCA-based rotation. Input points are now projected onto their best-fit plane.
- **Commit:** `8285c8e` - feat(v04-phase-b-structural-logic): enhance RBF engine for 3D fault modeling

## Task 3: Interactive Fault Picking
- **Goal:** Implement interactive fault picking and real-time modeling.
- **Outcome:** 
  - Added `SketchFault` picking mode.
  - Implemented click-and-drag sketching in `ViewportWidget`.
  - Wired sketch completion to `Fault::update_mesh()` which uses the 3D RBF engine.
  - Added 2D wireframe overlays for faults in the viewport for immediate feedback.
- **Verification:** 
  - Added unit test `test_fault_sketching` in `interpretation/mod.rs` to verify stick addition and mesh generation.
  - Verified `cargo check --workspace` passes.
- **Commit:** `feef8be` - feat(v04-phase-b-structural-logic): implement interactive fault picking and real-time modeling

## Deviations from Plan
- **Task 2 (Transparent Surface Rendering) was skipped** as the specific instruction was to execute Task 3.
- Added `FaultStick` and `Fault` helper methods to `InterpretationState` for better ergonomics.

## Known Stubs
- Task 2 implementation is still missing (Transparent Surface Rendering).
- 3D Rendering of faults is currently fallback to 2D wireframe overlays in `ViewportWidget`.

## Self-Check: PASSED
- [x] RBF adapt for 3D
- [x] Interactive Sketch Mode implemented
- [x] Real-time mesh updates wired
- [x] Unit tests passing
- [x] cargo check passed
