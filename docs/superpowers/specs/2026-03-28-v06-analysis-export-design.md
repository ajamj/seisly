# StrataForge v0.6: Analysis & Export Design

**Date:** 2026-03-28  
**Phase:** v0.6 - Analysis & Export  
**Status:** Approved by User (Route A/B delegated)

## 1. Overview
StrataForge v0.6 transitions the platform from a pure workstation into a results-driven engine. This phase focuses on calculating physical metrics (volume, throw) and exporting high-quality geological data for use in other software or reporting.

## 2. Key Components

### 2.1. Surface Clipping Engine (`sf_compute`)
- **Mesh-Surface Intersection:** Calculate the intersection line between horizons and faults. This line will be rendered as a new "Intersection" visual entity.
- **Hard Cutting:** Update the `Surface` domain model to support a collection of meshes. Implement logic to split a horizon mesh into separate discrete components when cut by a fault plane.
- **Throw Calculation:** Calculate the vertical displacement (throw) along the intersection line to quantify structural deformation.

### 2.2. Volumetric Calculator (`sf_compute`)
- **Gross Rock Volume (GRV):** Implement a grid-based engine to calculate the space between an Upper and Lower surface.
- **Analysis UI:** Implement a multi-selection mechanism in the Interpretation Explorer to allow the user to select two horizons for comparison and volume calculation.

### 2.3. Export Manager (`sf_io` + `sf_app`)
- **Geophysical Export:** Save interpreted horizons as XYZ point clouds.
- **Interoperability Export:** Export all interpretation data (picks, faults, horizons) as a structured **StrataForge JSON** file, preserving 3D coordinates, CRS, and geological metadata. (Avoid GeoJSON due to 2D/WGS84 constraints).
- **Mesh Export:** Provide basic support for exporting generated RBF surfaces as STL or OBJ files for external visualization.

### 2.4. UI Enhancements (`sf_app`)
- **Analysis Toolbar:** Dedicated buttons for "Calculate Volume" and "Export Data".
- **Result Overlay:** A floating panel to display computed statistics for the active interpretation.

## 3. Data Flow
1. **Select:** User selects an Upper and Lower horizon in the Interpretation Explorer.
2. **Compute:** `sf_compute` runs the volumetric integration and intersection clipping.
3. **Display:** Results (e.g., "GRV: 1.2M m³") are shown in the Analysis panel.
4. **Export:** User triggers an export; `sf_io` writes the selected interpretation to disk in the chosen format (XYZ/JSON).

## 4. Success Criteria
- [ ] Correctly calculate the volume between two parallel or intersecting surfaces.
- [ ] Successfully split a horizon mesh using a vertical or high-angle fault plane.
- [ ] Export a interpreted horizon to an XYZ file that can be read by other GIS tools.
- [ ] Save the entire project interpretation state to a single interoperable JSON file.
