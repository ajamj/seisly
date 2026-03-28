# StrataForge v0.4: Structural Interpretation Module Design

**Date:** 2026-03-28  
**Phase:** v0.4 - Advanced Features (Structural)  
**Status:** Approved by User

## 1. Overview
StrataForge v0.4 introduces advanced structural interpretation capabilities, moving beyond simple horizon tracking to complex fault modeling. The goal is to provide geoscientists with tools to interpret structural breaks (faults) and visualize them as semi-transparent 3D surfaces in real-time.

## 2. Key Components

### 2.1. Structural interpretation State (`sf_app`)
- **Fault Entity:** A new domain entity in the interpretation manager that stores a collection of "Fault Sticks" (sequences of picks).
- **Plane-First Logic:** Any addition or modification to a Fault Stick triggers an immediate update to the associated fault surface.

### 2.2. Enhanced RBF Engine (`sf_compute`)
- **Near-Vertical Modeling:** Adapt the RBF (Radial Basis Function) engine to handle high-angle and vertical fault planes, which typically challenge standard surface interpolation.
- **Stick-to-Surface Conversion:** Logic to turn sparse 3D sticks into a continuous mathematical representation of a fault plane.

### 2.3. Transparency & Depth Rendering (`sf_render`)
- **Alpha Blending:** Update the `wgpu` pipelines to support semi-transparent surface rendering.
- **Depth Sorting:** Implement basic depth-sorting for transparent objects to ensure correct visual alignment when viewing faults over seismic slices.

### 2.4. Structural UI Module (`sf_app`)
- **Structural Explorer:** A dedicated tab or tree-view in the interpretation panel for managing Fault systems.
- **Fault Picking Mode:** A specialized interaction mode that allows interpretors to "sketch" sticks on seismic sections.

## 3. Workflow (The Structural Loop)
1. **Define:** User creates a new "Fault Object" in the Structural Explorer.
2. **Pick:** User sketches sticks on multiple seismic slices.
3. **Model:** The RBF engine automatically generates a 3D surface from the available sticks.
4. **Verify:** User visualizes the semi-transparent fault plane overlaid on seismic data to verify geological alignment.

## 4. Technical Constraints
- **Performance:** Real-time modeling must be optimized to ensure the UI remains responsive during active picking.
- **Visualization:** Transparency requires careful handling of the render pass to avoid artifacts (e.g., OIT - Order Independent Transparency, or simple sorting).

## 5. Success Criteria
- [ ] Ability to create and manage Fault objects in the Interpretation panel.
- [ ] Functional "Fault Picking" mode for sketching sticks in the viewport.
- [ ] Real-time generation of smooth 3D fault surfaces from interpreted sticks.
- [ ] Successful rendering of semi-transparent fault planes over seismic slices.
