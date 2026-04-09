---
phase: v05-phase-a-horizon-picking
plan: 2026-03-28-v05-phase-a-horizon-picking.md
subsystem: [seisly_app, seisly_render, seisly_compute]
tags: [horizon, interpretation, picking, 3D, surface]
dependency_graph:
  requires: [v04-phase-c-structural-rendering, seisly_compute, seisly_render]
  provides: [Horizon interpretation workflow, surface visualization]
  affects: [seisly_app, seisly_render]
tech_stack:
  added: []
  patterns: [Horizon Picking, Surface Generation, Layer Management]
key_files:
  - crates/seisly_app/src/widgets/horizon_properties_panel.rs
  - crates/seisly_app/src/widgets/viewport.rs
  - crates/seisly_app/src/interpretation/mod.rs
  - crates/seisly_render/src/horizon_renderer.rs
decisions: []
metrics:
  duration: 1h
  completed_date: null
  tasks_total: 5
  tasks_completed: 0
---

# Phase v05-A Plan: Horizon Interpretation & Picking

## Overview
Implement comprehensive horizon interpretation workflow including manual picking, auto-tracking, surface generation, and 3D visualization with transparency.

## Objectives
1. Enhance horizon picking UI with better visual feedback
2. Add horizon surface rendering with transparency (similar to faults)
3. Create horizon properties panel for editing
4. Implement horizon layer management
5. Support multiple horizon visualization

---

## Task 1: Horizon Properties Panel & Layer Management

**Goal:** Create UI for editing horizon properties and managing layers

**Acceptance Criteria:**
- [ ] Horizon properties panel with: name, color picker, visibility toggle
- [ ] Layer list showing all horizons with checkboxes
- [ ] Click to select horizon for editing
- [ ] Delete horizon functionality
- [ ] Unit tests for property changes

**Files to Create/Modify:**
- `crates/seisly_app/src/widgets/horizon_properties_panel.rs` - Create new widget
- `crates/seisly_app/src/interpretation/mod.rs` - Add property edit methods
- `crates/seisly_app/src/app.rs` - Integrate properties panel in UI

**Implementation Steps:**
1. Create `HorizonPropertiesPanel` widget (similar to FaultPropertiesPanel)
   - Text input for horizon name
   - Color picker (egui's `ColorPicker`)
   - Visibility checkbox
   - Delete button
2. Create layer list widget
   - Show all horizons with name + color indicator
   - Checkbox for visibility toggle
   - Click to select
3. Add methods to `InterpretationState`:
   - Already has `set_color()`, `set_visible()`, `set_name()` from Fault
4. Wire up UI callbacks to state changes
5. Add unit tests

**UI Layout:**
```
┌─ Horizon Properties ──────────┐
│ Name: [Horizon A________]     │
│ Color: [████] RGB: 0,255,0    │
│ Visibility: [✓] Visible       │
│ [Apply] [Delete]              │
├─ Horizon Layers ──────────────┤
│ [✓] ████ Horizon A (selected) │
│ [✓] ████ Horizon B            │
│ [+ Add Horizon]               │
└───────────────────────────────┘
```

---

## Task 2: Horizon Renderer with Transparency

**Goal:** Create dedicated renderer for horizon surfaces

**Acceptance Criteria:**
- [ ] `HorizonRenderer` struct similar to `FaultRenderer`
- [ ] Transparent surface rendering with alpha blending
- [ ] MVP matrix support for 3D transformation
- [ ] Integration with viewport render loop
- [ ] `cargo check --workspace` passes

**Files to Create/Modify:**
- `crates/seisly_render/src/horizon_renderer.rs` - Create new renderer
- `crates/seisly_render/src/shaders/horizon.wgsl` - Create shader
- `crates/seisly_render/src/lib.rs` - Export HorizonRenderer
- `crates/seisly_app/src/widgets/viewport.rs` - Integrate renderer

**Implementation Steps:**
1. Create `horizon.wgsl` shader (can reuse fault.wgsl as template)
   - MVP uniforms
   - Alpha blending
   - Simple lighting
2. Create `HorizonRenderer` struct
   - Similar to `FaultRenderer`
   - `prepare_horizon()` method
   - `render()` method
3. Create `HorizonRenderData` struct
   - Bind group + uniform buffer
4. Export from `lib.rs`
5. Integrate into `ViewportCallback::prepare()`

**Technical Notes:**
- Can share shader code with fault renderer
- Horizon surfaces are typically more horizontal than faults
- May want different default transparency (70% vs 50%)

---

## Task 3: Enhanced Horizon Picking UI

**Goal:** Improve visual feedback during horizon picking

**Acceptance Criteria:**
- [ ] Show picked points with color-coded indicators
- [ ] Display horizon mesh wireframe in real-time
- [ ] Show pick count per horizon
- [ ] Highlight active horizon in viewport
- [ ] Unit tests for picking logic

**Files to Modify:**
- `crates/seisly_app/src/widgets/viewport.rs` - Enhance overlay rendering
- `crates/seisly_app/src/interpretation/mod.rs` - Add pick management methods

**Implementation Steps:**
1. Enhance `draw_overlays()` method
   - Draw picks as colored circles (using horizon color)
   - Draw mesh wireframe with transparency
   - Show pick count label
2. Add active horizon highlight
   - Thicker lines or glow effect
3. Add hover tooltip showing pick details
4. Test with existing picking modes (Manual, Seed, AutoTrack)

---

## Task 4: Horizon Auto-Tracking Enhancement

**Goal:** Improve auto-tracking algorithm usability

**Acceptance Criteria:**
- [ ] Visual feedback during auto-tracking progress
- [ ] Cancel auto-tracking mid-operation
- [ ] Display tracking statistics (points tracked, confidence)
- [ ] Unit tests for tracking logic

**Files to Modify:**
- `crates/seisly_app/src/widgets/viewport.rs` - Add progress indicator
- `crates/seisly_compute/src/tracking.rs` - Add progress callback

**Implementation Steps:**
1. Add tracking progress state
2. Show progress bar during auto-track
3. Add cancel button
4. Display results summary after completion

---

## Task 5: Multiple Horizon Visualization

**Goal:** Support visualization of multiple horizons simultaneously

**Acceptance Criteria:**
- [ ] All visible horizons rendered in 3D
- [ ] Proper depth sorting for transparency
- [ ] Color-coded horizon identification
- [ ] Performance acceptable with 5+ horizons
- [ ] Unit tests for multi-horizon rendering

**Files to Modify:**
- `crates/seisly_app/src/widgets/viewport.rs` - Multi-horizon render loop
- `crates/seisly_render/src/horizon_renderer.rs` - Batch rendering support

**Implementation Steps:**
1. Iterate through all visible horizons
2. Prepare render data for each
3. Render with proper depth sorting (back-to-front)
4. Optimize with batch rendering if needed

---

## Verification

### Self-Check Checklist
- [ ] All 5 tasks completed
- [ ] Each task committed individually with descriptive messages
- [ ] `cargo check --workspace` passes
- [ ] Unit tests added and passing
- [ ] SUMMARY.md created with decisions and deviations

### Acceptance Criteria (Phase Level)
- [ ] Horizons render as transparent 3D surfaces
- [ ] User can change horizon color via UI
- [ ] User can toggle horizon visibility
- [ ] User can rename horizons
- [ ] Multiple horizons can be visualized simultaneously
- [ ] Picking workflow is intuitive and responsive

---

## Dependencies

**Requires:**
- v04-phase-c-structural-rendering (Fault rendering infrastructure)
- seisly_render crate (wgpu infrastructure)
- seisly_compute (RBF interpolation, auto-tracking)

**Provides:**
- Complete horizon interpretation workflow
- Foundation for velocity modeling (v05-phase-b)

---

## Risk & Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Performance with many horizons | High | Implement LOD, batch rendering |
| Transparency sorting artifacts | Medium | Use depth pre-pass or OIT |
| Auto-tracking accuracy | Medium | Provide manual override, confidence display |
