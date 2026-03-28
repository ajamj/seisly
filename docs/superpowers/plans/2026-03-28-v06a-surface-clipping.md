# StrataForge v0.6a: Surface Clipping & Throw Calculation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement geometric tools to calculate surface intersections, split meshes along fault planes, and quantify vertical throw.

**Architecture:** Update `Surface` to hold a `Vec<Mesh>`. Implement a `SurfaceClippingEngine` in `sf_compute` that uses triangle-plane intersection logic. Add a `ThrowCalculator` to measure vertical separation between split mesh components.

**Tech Stack:** Rust, nalgebra.

---

### Task 1: Multi-Mesh Surface Support (Refactor)

**Files:**
- Modify: `crates/sf_core/src/domain/surface.rs`
- Modify: `crates/sf_storage/src/project.rs`
- Modify: `crates/sf_render/src/mesh.rs`
- Modify: `crates/sf_app/src/interpretation/mod.rs`

- [ ] **Step 1: Refactor Surface to support multiple meshes in sf_core**

```rust
pub struct Surface {
    pub metadata: DatasetMetadata,
    pub meshes: Vec<Mesh>, 
    pub intersection_lines: Vec<Vec<[f32; 3]>>, // New visual entity
}
```

- [ ] **Step 2: Update sf_storage and sf_render to handle multiple meshes**
Ensure the renderer loops through all meshes in a surface.

- [ ] **Step 3: Update InterpretationState in sf_app**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_core/ crates/sf_storage/ crates/sf_render/ crates/sf_app/
git commit -m "refactor: support multiple meshes and intersection lines in Surface model"
```

---

### Task 2: Mesh-Surface Intersection Engine

**Files:**
- Create: `crates/sf_compute/src/clipping.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [ ] **Step 1: Implement triangle-plane intersection**
Find the intersection segment between a 3D triangle and an infinite plane.

- [ ] **Step 2: Implement full mesh intersection**
Collect all segments into a polyline. Store in `Surface::intersection_lines`.

- [ ] **Step 3: Add unit tests for horizontal vs vertical intersection**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_compute/src/clipping.rs crates/sf_compute/src/lib.rs
git commit -m "feat: implement mesh-surface intersection engine"
```

---

### Task 3: Hard Cutting & Mesh Splitting

**Files:**
- Modify: `crates/sf_compute/src/clipping.rs`

- [ ] **Step 1: Implement mesh splitting logic with epsilon stability**
Divide a mesh into two separate meshes (labeled Up-thrown/Down-thrown). Use a small epsilon for vertices on the plane.

- [ ] **Step 2: Handle triangles that cross the plane**
Clip triangles and generate new geometry for both sides.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_compute/src/clipping.rs
git commit -m "feat: implement mesh splitting (hard cutting) along a plane"
```

---

### Task 4: Throw Distribution Calculation

**Files:**
- Create: `crates/sf_compute/src/throw.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [ ] **Step 1: Implement throw calculation logic**
Calculate vertical displacement at discrete points along the intersection line.

- [ ] **Step 2: Add unit tests including blind faults**

- [ ] **Step 3: Commit**

```bash
git add crates/sf_compute/src/throw.rs crates/sf_compute/src/lib.rs
git commit -m "feat: implement vertical throw distribution calculation"
```
