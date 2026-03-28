# StrataForge v0.5: Depth Conversion & Multi-Volume Design

**Date:** 2026-03-28  
**Phase:** v0.5 - Depth Conversion & Multi-Volume  
**Status:** Approved by User

## 1. Overview
StrataForge v0.5 expands the workstation to support complex multi-volume workflows and transitions from the seismic time domain (ms) to the depth domain (m). This allows for accurate structural modeling and integration with well data in true physical coordinates.

## 2. Key Components

### 2.1. Velocity Modeling Engine (`sf_compute`)
- **Linear Velocity Model ($V_0 + kZ$):** Implement the standard analytical conversion for instantaneous linear velocity. Formula: $Z(t) = \frac{V_0}{k} (e^{\frac{kt}{2}} - 1)$, where $t$ is Two-Way Time (TWT) in seconds.
- **Conversion API:** Provide methods for on-the-fly projection of 3D picks and surfaces from Time to Depth.

### 2.2. Multi-Volume Manager (`sf_storage` + `sf_app`)
- **Seismic Registry:** Update the project state to manage multiple SEG-Y volumes per project.
- **Toggled Visibility:** A UI in the Project Data panel to switch between active volumes (e.g., Near, Mid, Far stacks).
- **RGB Blending (2D Only):** Implement attribute blending for 2D slices (Inline/Crossline) by mapping up to three active volumes to R, G, and B color channels.

### 2.3. Depth-Aware 3D Viewport (`sf_render` + `sf_app`)
- **Vertical Axis Toggle:** Switch between "Time Mode" (ms) and "Depth Mode" (m).
- **Interpretation Projection:** Interpretation data (horizons, faults, wells) is projected to depth in real-time based on the active velocity parameters.
- **Seismic Display in Depth:** In this initial phase, seismic volume rendering will be disabled when in Depth Mode. Seismic data remains tied to the Time domain.

### 2.4. UI Enhancements (`sf_app`)
- **Velocity Toolbar:** New controls to set $V_0$ and $k$ parameters.
- **Multi-Volume Explorer:** Tree-view improvements to handle nested seismic datasets.

## 3. Data Flow
1. **Model:** User inputs $V_0$ and $k$ in the Velocity Toolbar.
2. **Transform:** `sf_compute` dynamically projects the interpretation geometry from Time to Depth.
3. **Switch:** User toggles the "Depth Mode" in the viewport.
4. **Render:** `sf_render` displays interpretation surfaces and well trajectories in physical depth coordinates; seismic slices are hidden in this mode.

## 4. Success Criteria
- [ ] Successful transformation of interpretation horizons from Time to Depth using $V_0 + kZ$.
- [ ] Seamless switching and RGB blending of multiple seismic volumes in 2D slices.
- [ ] Integrated UI for managing linear velocity model parameters.
- [ ] Viewport supports toggling between TWT (Time) and Depth units for interpretation data.
