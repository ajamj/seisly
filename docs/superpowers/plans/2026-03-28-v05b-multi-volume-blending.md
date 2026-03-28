# StrataForge v0.5b: Multi-Volume Management & RGB Blending Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable management of multiple seismic volumes and support RGB blending for 2D attribute analysis.

**Architecture:** Extend `Project` state to store a list of seismic datasets. Implement a `MultiVolumeManager` in `sf_app` to handle selection. Update `sf_render` to support drawing multiple textures with blend modes.

**Tech Stack:** Rust, wgpu.

---

### Task 1: Multi-Volume Registry

**Files:**
- Modify: `crates/sf_storage/src/project.rs`
- Modify: `schemas/sqlite/0001_init.sql`

- [ ] **Step 1: Update SQL schema for multiple datasets**

- [ ] **Step 2: Update ProjectManifest to include a list of volumes**

- [ ] **Step 3: Commit**

```bash
git add schemas/sqlite/0001_init.sql crates/sf_storage/src/project.rs
git commit -m "feat: add multi-volume support to project state"
```

---

### Task 2: Multi-Volume UI (Explorer)

**Files:**
- Modify: `crates/sf_app/src/app.rs`

- [ ] **Step 1: Update Project Data panel to list all volumes**

- [ ] **Step 2: Add check-boxes for R, G, B channel assignment**

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/app.rs
git commit -m "ui: implement multi-volume explorer with channel assignment"
```

---

### Task 3: RGB Blending Shader (2D)

**Files:**
- Modify: `crates/sf_render/src/shaders/basic.wgsl`
- Modify: `crates/sf_render/src/renderer.rs`

- [ ] **Step 1: Update shader to sample up to 3 textures**

```wgsl
// In basic.wgsl:
@group(0) @binding(0) var t_red: texture_2d<f32>;
@group(0) @binding(1) var t_green: texture_2d<f32>;
@group(0) @binding(2) var t_blue: texture_2d<f32>;
```

- [ ] **Step 2: Implement RGB combination logic in fragment shader**

- [ ] **Step 3: Update Renderer to handle multiple bind groups**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_render/src/
git commit -m "feat: implement RGB attribute blending shader"
```
