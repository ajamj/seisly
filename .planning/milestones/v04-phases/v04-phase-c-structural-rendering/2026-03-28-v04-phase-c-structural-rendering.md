---
phase: v04-phase-c-structural-rendering
plan: 2026-03-28-v04-phase-c-structural-rendering.md
subsystem: [seisly_render, seisly_app]
tags: [3D, rendering, wgpu, transparency, UI]
dependency_graph:
  requires: [v04-phase-b-structural-logic, seisly_render, seisly_compute]
  provides: [3D fault visualization, fault properties UI]
  affects: [seisly_app, seisly_render]
tech_stack:
  added: []
  patterns: [Transparent Rendering, Property Panel Pattern]
key_files:
  - crates/seisly_render/src/fault_renderer.rs
  - crates/seisly_app/src/widgets/fault_properties_panel.rs
  - crates/seisly_app/src/interpretation/mod.rs
  - crates/seisly_app/src/widgets/viewport.rs
decisions: []
metrics:
  duration: 45m
  completed_date: null
  tasks_total: 3
  tasks_completed: 0
---

# Phase v04-C Plan: Structural Rendering (3D Fault Visualization)

## Overview
Complete the deferred transparent surface rendering task and build a comprehensive 3D fault visualization system with interactive property editing.

## Objectives
1. Implement 3D transparent fault surface rendering using wgpu
2. Add fault properties panel for editing name, color, visibility
3. Enable layer management for multiple faults
4. Provide 2D/3D toggle for fault visualization modes

---

## Task 1: Complete Deferred Task 2 - Transparent Surface Rendering Foundation

**Goal:** Implement the skeleton for transparent fault surface rendering that was deferred from v04-phase-b

**Acceptance Criteria:**
- [ ] `Fault` struct has `color: [f32; 4]` field (RGBA with alpha)
- [ ] Default fault color with 0.5 alpha for transparency
- [ ] `FaultMesh` struct ready for wgpu rendering
- [ ] Unit test for fault color initialization

**Files to Create/Modify:**
- `crates/seisly_compute/src/interpolation.rs` - Add color field to Fault
- `crates/seisly_render/src/fault_renderer.rs` - Create new renderer

**Implementation Steps:**
1. Add `color` and `visible` fields to `Fault` struct
2. Create `FaultMesh` struct to hold vertex/index buffers
3. Create `FaultRenderer` struct with wgpu pipeline for transparent rendering
4. Add unit test for fault color defaults

---

## Task 2: 3D Fault Mesh Rendering with wgpu

**Goal:** Render fault surfaces as transparent 3D meshes in the viewport

**Acceptance Criteria:**
- [ ] Faults render as transparent surfaces in 3D viewport
- [ ] Alpha blending enabled for proper transparency
- [ ] Depth sorting or order-independent transparency
- [ ] Wireframe overlay option for better visibility
- [ ] `cargo check --workspace` passes

**Files to Create/Modify:**
- `crates/seisly_render/src/fault_renderer.rs` - Implement render logic
- `crates/seisly_app/src/widgets/viewport.rs` - Integrate fault renderer
- `crates/seisly_render/src/lib.rs` - Export FaultRenderer

**Implementation Steps:**
1. Create wgpu render pipeline for transparent triangles
   - Vertex shader: position + normal
   - Fragment shader: output color with alpha
   - Blend state: alpha blending (SrcAlpha, OneMinusSrcAlpha)
2. Generate vertex buffers from RBF mesh triangles
3. Integrate `FaultRenderer` into `ViewportWidget`
4. Add render pass for faults after seismic volume rendering
5. Test with existing fault stick data

**Technical Notes:**
- Use alpha blending: `Color::OVER` or `BlendState::ALPHA_BLENDING`
- Render faults after opaque objects (seismic) for correct blending
- Consider depth bias to prevent z-fighting with wireframe overlay

---

## Task 3: Fault Properties Panel & Layer Management

**Goal:** Add UI for editing fault properties and managing multiple fault layers

**Acceptance Criteria:**
- [ ] Fault properties panel with: name, color picker, visibility toggle
- [ ] Layer list showing all faults with checkboxes
- [ ] Click to select fault for editing
- [ ] Changes persist to SQLite database
- [ ] Unit tests for property changes

**Files to Create/Modify:**
- `crates/seisly_app/src/widgets/fault_properties_panel.rs` - Create new widget
- `crates/seisly_app/src/interpretation/mod.rs` - Add property edit methods
- `crates/seisly_storage/src/sqlite/mod.rs` - Add update fault query
- `crates/seisly_app/src/app.rs` - Integrate properties panel in UI

**Implementation Steps:**
1. Create `FaultPropertiesPanel` widget
   - Text input for fault name
   - Color picker (egui's `ColorPicker` or simple RGB sliders)
   - Visibility checkbox
   - Delete button
2. Create layer list widget
   - Show all faults with name + color indicator
   - Checkbox for visibility toggle
   - Click to select
3. Add methods to `InterpretationState`:
   - `update_fault_color(fault_id, color)`
   - `update_fault_name(fault_id, name)`
   - `toggle_fault_visibility(fault_id)`
   - `delete_fault(fault_id)`
4. Add SQLite update queries in `seisly_storage`
5. Wire up UI callbacks to state changes
6. Add unit tests

**UI Layout:**
```
┌─ Fault Properties ────────────┐
│ Name: [Fault A__________]     │
│ Color: [████] RGB: 128,0,0    │
│ Visibility: [✓] Visible       │
│ [Apply] [Delete]              │
├─ Fault Layers ────────────────┤
│ [✓] ████ Fault A (selected)   │
│ [✓] ████ Fault B              │
│ [ ] ████ Fault C              │
│ [+ Add Fault]                 │
└───────────────────────────────┘
```

---

## Verification

### Self-Check Checklist
- [ ] All 3 tasks completed
- [ ] Each task committed individually with descriptive messages
- [ ] `cargo check --workspace` passes
- [ ] Unit tests added and passing
- [ ] SUMMARY.md created with decisions and deviations

### Acceptance Criteria (Phase Level)
- [ ] Faults render as transparent 3D surfaces
- [ ] User can change fault color via UI
- [ ] User can toggle fault visibility
- [ ] User can rename faults
- [ ] Changes persist after app restart

---

## Dependencies

**Requires:**
- v04-phase-b-structural-logic (RBF interpolation, fault picking)
- seisly_render crate (wgpu infrastructure)
- egui integration in seisly_app

**Provides:**
- Complete 3D fault visualization
- Foundation for horizon rendering (v05-phase-a)

---

## Risk & Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Transparency sorting issues | Visual artifacts | Use depth pre-pass or order-independent transparency |
| wgpu pipeline complexity | Time overrun | Start with simple opaque rendering, add transparency after |
| SQLite schema changes | Migration needed | Add migration script or version check |
