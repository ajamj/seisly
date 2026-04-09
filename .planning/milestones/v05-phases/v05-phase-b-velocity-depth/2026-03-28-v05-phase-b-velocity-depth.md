---
phase: v05-phase-b-velocity-depth
plan: 2026-03-28-v05-phase-b-velocity-depth.md
subsystem: [seisly_app, seisly_compute]
tags: [velocity, depth, time-to-depth, conversion, modeling]
dependency_graph:
  requires: [v05-phase-a-horizon-picking, seisly_compute]
  provides: [Velocity modeling, depth conversion, depth domain visualization]
  affects: [seisly_app, seisly_compute]
tech_stack:
  added: [velocity model UI, depth conversion engine]
  patterns: [Velocity Modeling, Time-Depth Conversion]
key_files:
  - crates/seisly_app/src/widgets/velocity_panel.rs
  - crates/seisly_compute/src/velocity/depth_conversion.rs
  - crates/seisly_app/src/widgets/viewport.rs
decisions: []
metrics:
  duration: 1h
  completed_date: null
  tasks_total: 4
  tasks_completed: 0
---

# Phase v05-B Plan: Velocity & Depth Modeling

## Overview
Implement velocity modeling and time-to-depth conversion capabilities for seismic interpretation. This enables geoscientists to convert interpreted horizons from time domain (TWT) to depth domain.

## Objectives
1. Create velocity model UI for defining velocity functions
2. Implement time-to-depth conversion engine
3. Add depth domain visualization in viewport
4. Provide velocity analysis tools

---

## Task 1: Velocity Model UI & Visualization

**Goal:** Create UI for defining and editing velocity models

**Acceptance Criteria:**
- [ ] Velocity model panel with model type selector (Constant, Gradient, Layered)
- [ ] Parameter inputs for each model type:
  - Constant: V0 (single velocity value)
  - Gradient: V0 and k (gradient)
  - Layered: Multiple layers with thickness and velocity
- [ ] Visual preview of velocity function (V vs Depth curve)
- [ ] Apply/Save button to update conversion
- [ ] Unit tests for UI components

**Files to Create/Modify:**
- `crates/seisly_app/src/widgets/velocity_panel.rs` - Create new widget
- `crates/seisly_compute/src/velocity/model.rs` - Extend velocity model types
- `crates/seisly_app/src/app.rs` - Integrate velocity panel

**Implementation Steps:**
1. Create `VelocityPanel` widget
   - Model type dropdown (Constant, Gradient, Layered)
   - Dynamic parameter inputs based on type
   - Preview plot (simple ASCII or egui plot)
   - Apply/Save button
2. Extend `VelocityState` with new model types
3. Add velocity model persistence (save to project)
4. Wire up UI to velocity state changes

**UI Layout:**
```
┌─ Velocity Model ──────────────┐
│ Model Type: [Gradient ▼]      │
│                               │
│ Parameters:                   │
│ V0 (m/s): [2000____]          │
│ k (1/s):  [0.5_____]          │
│                               │
│ Preview:                      │
│ Depth │ Velocity              │
│   0   │  2000 m/s             │
│ 1000  │  2500 m/s             │
│ 2000  │  3000 m/s             │
│                               │
│ [Apply] [Reset]               │
└───────────────────────────────┘
```

---

## Task 2: Time-to-Depth Conversion Engine

**Goal:** Implement robust time-to-depth conversion algorithms

**Acceptance Criteria:**
- [ ] Support for Constant velocity model
- [ ] Support for Gradient velocity model (V = V0 + k*z)
- [ ] Support for Layered velocity model
- [ ] Conversion functions:
  - `time_to_depth(twt, velocity_model) -> depth`
  - `depth_to_time(depth, velocity_model) -> twt`
- [ ] Unit tests for each conversion type
- [ ] Edge case handling (negative values, zero velocity)

**Files to Create/Modify:**
- `crates/seisly_compute/src/velocity/depth_conversion.rs` - Create conversion engine
- `crates/seisly_compute/src/velocity/model.rs` - Define model types

**Implementation Steps:**
1. Define velocity model enum:
   ```rust
   pub enum VelocityModel {
       Constant { v0: f32 },
       Gradient { v0: f32, k: f32 },
       Layered { layers: Vec<Layer> },
   }
   ```
2. Implement conversion formulas:
   - Constant: `depth = v0 * twt / 2`
   - Gradient: `depth = (sqrt(V0² + 2*k*v0*twt) - V0) / k`
   - Layered: Sum of layer conversions
3. Add inverse conversion (depth to time)
4. Handle edge cases and validation

**Technical Notes:**
- TWT (Two-Way Time) in seconds
- Depth in meters
- Velocity in m/s
- Gradient k in 1/s (s⁻¹)

---

## Task 3: Depth Domain Visualization

**Goal:** Enable viewport to display data in depth domain

**Acceptance Criteria:**
- [ ] Depth Mode toggle in viewport
- [ ] Real-time conversion of picks from time to depth
- [ ] Depth scale on Y-axis (meters instead of TWT)
- [ ] Horizon mesh displayed at correct depth positions
- [ ] Unit tests for depth projection

**Files to Modify:**
- `crates/seisly_app/src/widgets/viewport.rs` - Add depth mode rendering
- `crates/seisly_app/src/interpretation/mod.rs` - Store picks in both domains

**Implementation Steps:**
1. Add `is_depth_mode` flag to ViewportWidget
2. Implement `sample_to_depth()` projection function
3. Modify `draw_overlays()` to use depth coordinates when enabled
4. Add depth scale labels on Y-axis
5. Update horizon mesh generation to use depth coordinates

**Visual Changes:**
```
Time Mode:          Depth Mode:
Y-axis: TWT (ms)    Y-axis: Depth (m)
  0 ─────             0 ─────
  500 ────            1000 ───
  1000 ───            2000 ───
  1500 ───            3000 ───
  2000 ───            4000 ───
```

---

## Task 4: Velocity Analysis Tools

**Goal:** Provide tools for velocity model calibration

**Acceptance Criteria:**
- [ ] Well tie visualization (if well data available)
- [ ] Velocity scan tool (pick velocity at different depths)
- [ ] Misfit analysis (compare predicted vs observed)
- [ ] Export velocity model to file
- [ ] Unit tests for analysis tools

**Files to Create/Modify:**
- `crates/seisly_app/src/widgets/velocity_analysis.rs` - Analysis tools
- `crates/seisly_compute/src/velocity/analysis.rs` - Analysis algorithms

**Implementation Steps:**
1. Create velocity scan widget
   - Click on seismic to pick velocity
   - Display picked velocity vs depth
2. Implement misfit calculation
   - Compare converted depth to known marker
   - Display RMS error
3. Add export functionality
   - Export velocity model as JSON or ASCII
4. (Future) Well tie integration

---

## Verification

### Self-Check Checklist
- [ ] All 4 tasks completed
- [ ] Each task committed individually
- [ ] `cargo check --workspace` passes
- [ ] Unit tests added and passing
- [ ] SUMMARY.md created

### Acceptance Criteria (Phase Level)
- [ ] User can define velocity model (Constant/Gradient/Layered)
- [ ] Time-to-depth conversion works correctly
- [ ] Depth mode visualization displays correctly
- [ ] Velocity analysis tools functional

---

## Dependencies

**Requires:**
- v05-phase-a-horizon-picking (horizon interpretation)
- seisly_compute (RBF interpolation foundation)

**Provides:**
- Depth-converted horizons
- Velocity model management
- Foundation for well integration (v06)

---

## Risk & Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Complex velocity math | High | Extensive unit tests, validate against known formulas |
| Performance with layered models | Medium | Cache conversion results, limit layer count |
| User confusion (time vs depth) | Medium | Clear UI labels, visual indicators |
