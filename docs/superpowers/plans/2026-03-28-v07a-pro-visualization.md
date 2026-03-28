# StrataForge v0.7a: Pro-Grade Visualization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement industry-standard seismic color mapping and a GPU-based ray-casting engine for semi-transparent 3D volume rendering, with high-fidelity UI polish.

**Architecture:** Add a `ColormapManager` in `sf_render` using 1D textures. Implement a 3D volumetric shader in `volumetric.wgsl` with mip-map acceleration. Update the `Renderer` to manage multi-sampled transparency. Enhance the UI with contextual toolbars and High-DPI icons.

**Tech Stack:** Rust, wgpu (WGSL), egui.

---

### Task 1: Pro-Grade Color Maps (LUTs)

**Files:**
- Create: `crates/sf_render/src/colormaps.rs`
- Modify: `crates/sf_render/src/lib.rs`

- [ ] **Step 1: Implement ColormapManager with presets**
Include: *Seismic (RWB)*, *Viridis*, *Magma*, and *Gray*.

- [ ] **Step 2: Implement GPU LUT texture generation**
Create 1D textures for each preset.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_render/src/colormaps.rs
git commit -m "feat: implement dynamic color mapping with Magma and Seismic presets"
```

---

### Task 2: Accelerated 3D Volumetric Ray-Caster

**Files:**
- Create: `crates/sf_render/src/shaders/volumetric.wgsl`
- Modify: `crates/sf_render/src/renderer.rs`

- [ ] **Step 1: Implement 3D Mip-map generation for seismic volumes**
Use `wgpu` to generate mip-maps for spatial acceleration during ray-casting.

- [ ] **Step 2: Implement ray-marching logic in volumetric.wgsl**
Sample mip-mapped 3D textures and apply transfer functions.

- [ ] **Step 3: Update Renderer to include Volumetric Pipeline**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_render/src/
git commit -m "feat: implement accelerated GPU ray-casting for 3D volumes"
```

---

### Task 3: Contextual UI & High-DPI Polish

**Files:**
- Modify: `crates/sf_app/src/app.rs`
- Create: `crates/sf_app/assets/icons/`

- [ ] **Step 1: Add High-DPI SVG icons for interpretation modes**

- [ ] **Step 2: Implement Contextual Toolbars**
Toolbar content updates dynamically based on the active selection (Horizon vs. Fault).

- [ ] **Step 3: Add Gain/Clip/Opacity sliders**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_app/
git commit -m "ui: implement contextual toolbars and high-DPI icons"
```
