# StrataForge v0.5a: Velocity Modeling & Depth Conversion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a linear velocity model ($V_0 + kZ$) and enable real-time depth projection for all interpretation data (picks, surfaces, wells) in the 3D viewport.

**Architecture:** A stateless `LinearVelocityModel` in `sf_compute` for TWT-to-Depth conversion. A stateful `VelocityState` in `sf_app` to manage parameters and unit scaling. The `ViewportWidget` will be updated to support vertical slicing and on-the-fly projection.

**Tech Stack:** Rust, nalgebra.

---

### Task 1: Enhanced Linear Velocity Model (sf_compute)

**Files:**
- Create: `crates/sf_compute/src/velocity.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [ ] **Step 1: Implement the LinearVelocityModel with unit scaling**

```rust
pub struct LinearVelocityModel {
    pub v0: f32,           // Initial velocity at Z=0 (m/s)
    pub k: f32,            // Acceleration gradient (s^-1)
    pub sample_rate_ms: f32, // ms per sample for unit conversion
    pub start_time_ms: f32,  // T0 offset
}

impl LinearVelocityModel {
    /// Convert sample index to Depth (Z) in meters.
    pub fn sample_to_depth(&self, sample_idx: f32) -> f32 {
        let twt_sec = (self.start_time_ms + sample_idx * self.sample_rate_ms) / 1000.0;
        if self.k.abs() < 1e-6 {
            self.v0 * twt_sec / 2.0
        } else {
            (self.v0 / self.k) * ((self.k * twt_sec / 2.0).exp() - 1.0)
        }
    }
}
```

- [ ] **Step 2: Add unit tests for sample-to-depth conversion**

- [ ] **Step 3: Commit**

```bash
git add crates/sf_compute/src/velocity.rs crates/sf_compute/src/lib.rs
git commit -m "feat: implement enhanced linear velocity model with unit scaling"
```

---

### Task 2: Vertical Slice View (Viewport)

**Files:**
- Modify: `crates/sf_app/src/widgets/viewport.rs`

- [ ] **Step 1: Add ViewMode toggle (Map vs Section)**
Update the widget to support rendering Inline vs. Sample (Section view) instead of just Inline vs. Crossline (Map view).

- [ ] **Step 2: Update coordinate projection**
Ensure clicks in Section view map correctly to Inline/Crossline/Sample coordinates.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/widgets/viewport.rs
git commit -m "ui: implement vertical section view in viewport"
```

---

### Task 3: Depth UI & Full Projection Logic

**Files:**
- Create: `crates/sf_app/src/interpretation/velocity.rs`
- Modify: `crates/sf_app/src/app.rs`
- Modify: `crates/sf_app/src/widgets/viewport.rs`

- [ ] **Step 1: Implement VelocityState and Toolbar**
Include $V_0, k, sample\_rate, start\_time$.

- [ ] **Step 2: Update Viewport to project Picks, Surfaces, and Wells**
When `is_depth_mode` is true, apply `sample_to_depth` to the $Z$ component of all interpretation geometry during the `draw_overlays` pass.

- [ ] **Step 3: Hide seismic slices in Depth Mode**
(As per v0.5 spec constraints).

- [ ] **Step 4: Commit**

```bash
git add crates/sf_app/src/ crates/sf_app/src/widgets/viewport.rs
git commit -m "feat: implement real-time depth projection for interpretation data"
```
