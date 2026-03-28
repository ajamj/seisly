# StrataForge v0.7: High-Fidelity Workstation Design

**Date:** 2026-03-28  
**Phase:** v0.7 - Advanced Visualization & Polish  
**Status:** Approved by User (Route: "Better than Petrel")

## 1. Overview
StrataForge v0.7 aims to deliver a visualization and interaction experience that exceeds industry standards. This phase focuses on high-end GPU rendering techniques (ray-casting), sophisticated color management, and a robust undo/redo system to ensure professional-grade productivity and visual clarity.

## 2. Key Components

### 2.1. Dynamic Color Mapping Engine (`sf_render`)
- **LUT Textures:** Implement 1D Look-Up Tables (LUTs) for seismic color mapping.
- **Pro Presets:** Include industry-standard maps: *Seismic (Red-White-Blue)*, *Viridis*, *Magma*, and *Standard Gray*.
- **Real-time Gain & Clip:** Add GPU-side scaling for amplitude contrast and clipping to highlight specific geological features.

### 2.2. 3D Volume Ray-Caster (`sf_render`)
- **GPU Ray-casting:** Implement a specialized shader to render the seismic cube as a semi-transparent volume.
- **Transfer Functions:** Allow the user to map seismic amplitude to both color and transparency (opacity), enabling them to "see through" low-amplitude noise to high-contrast events.

### 2.3. Well-Log Visualization (`sf_render` + `sf_io`)
- **Log Strips in 3D:** Render well logs (e.g., Gamma Ray, Resistivity) as interactive "strips" or "tubes" along the 3D trajectory.
- **Cross-plot Widget:** Implement a basic 2D cross-plot panel (e.g., Gamma vs. Depth) using `egui_plot` to allow statistical data analysis. Clicking a point on a log in the 3D view highlights the corresponding data in the cross-plot.

### 2.4. Interpretation Undo/Redo Stack (`sf_app`)
- **Action-Based History:** Implement a command pattern to track every interpretation action (picks, auto-tracks, surface generation).
- **Session-Persistent History:** Store the command stack in-memory during the session. (Disk persistence via SQLite is planned for a future maintenance phase).

### 2.5. UI/UX Polish (`sf_app`)
- **High-DPI Support:** Ensure all icons and UI elements are crisp on 4K/modern displays.
- **Contextual Toolbars:** Smart toolbars that change based on whether the user is picking a Horizon, a Fault, or analyzing Volumetrics.

## 3. Technical Strategy
1. **Shaders:** Update `wgpu` shaders to handle multi-sampling and complex blending for ray-casting. Use mip-mapping or sparse voxel structures for spatial acceleration.
2. **State:** Refactor `InterpretationState` to use an atomic command-buffer approach for Undo/Redo.
3. **Performance:** Utilize `rayon` for parallelizing well-log projection into 3D space.

## 4. Success Criteria
- [ ] Silky-smooth 3D volume rendering with adjustable transparency.
- [ ] Instant switching between different seismic color maps without re-loading data.
- [ ] Full Undo/Redo support for all interpretation clicks and edits.
- [ ] Professional-grade 3D well-log visualization aligned with seismic data.
