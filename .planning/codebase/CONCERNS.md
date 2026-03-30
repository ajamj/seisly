# Codebase Concerns & Strategic Gaps

## 1. Technical Debt & Implementation Gaps

### Core & Domain (`seisly_core`)
- **Surface Normals:** Proper normal calculation for surfaces is still pending (currently a TODO). This affects rendering quality and lighting.
- **Data Integrity:** BLAKE3 hashing is implemented for blobs, but full-project integrity checks are not yet automated.

### Compute & Algorithms (`seisly_compute`)
- **Clipping Engine:** Clipping currently returns individual segments rather than connected polylines. This makes further analysis (like throw calculation) more difficult.
- **Interpolation:** Proper ray-casting or barycentric interpolation for exact Z-value retrieval on surfaces is missing.
- **FWI (Full Waveform Inversion):** The `seisly_fwi` crate is largely scaffolded. Missing:
    - Full elastic misfit and gradient calculations.
    - Velocity update logic.
    - Adjoint state method implementation for acoustic/elastic kernels.

### Machine Learning (`seisly_ml_deep`)
- **Model Lifecycle:** Model saving and loading logic is incomplete (uses `todo!` macros).
- **Data Augmentation:** Essential for robust ML training, but currently marked as a TODO.
- **Connectivity Analysis:** Fault detection lacks connectivity analysis to group voxels into distinct fault planes.
- **Throw Calculation:** Automatic calculation of vertical throw along faults is not yet implemented.

### Plugin System (`seisly_plugin`)
- **Discovery:** Automatic plugin discovery from a specific directory is missing; plugins currently must be loaded manually by path.

## 2. Performance & Scalability

### Memory Footprint
- Large seismic volumes (e.g., 1024x1024x512) require 32GB+ of RAM. While memory-mapping (`memmap2`) helps, processing still hits physical memory limits on standard workstations.
- Need for a more robust "out-of-core" processing strategy for very large datasets.

### Processing Latency
- 1024x1024x512 volume tracking takes ~10 minutes.
- GPU acceleration is currently limited to a subset of attributes (RMS, Mean, Energy). Expanding this to frequency and geometric attributes (Dip/Azimuth) is critical for "Pro" performance.

## 3. Security & Enterprise Readiness

### Authentication & Authorization
- Currently offline-only. The planned "Server Mode" (v2.0) lacks:
    - OIDC/SSO integration.
    - Role-based access control (RBAC) at the project or dataset level.
    - Audit logging for data modifications.

### Data Privacy
- No encryption-at-rest for the blob store or SQLite metadata.

## 4. Strategic Feature Gaps (v0.6.0 → v1.0.0)

- **Undo/Redo:** Missing a global undo/redo stack for interpretation picks, which is essential for user productivity.
- **3D Volume Rendering:** Currently limited to 2D slices. True ray-casting volume rendering is planned but not implemented.
- **Well-Seismic Tie:** Visualization is present, but the automated stretch/squeeze workflow for time-depth adjustment is in early stages.
- **Color Maps:** Lack of customizable or industry-standard color maps (Viridis, Seismic, etc.) beyond basic presets.

## 5. Testing & Quality Assurance
- **Coverage:** Grep suggests many `todo!` and `TODO` markers in deep-learning and FWI modules, indicating low unit test coverage in high-complexity areas.
- **Integration Tests:** Need for more E2E tests covering the full workflow from SEG-Y import to ML-based horizon picking and final export.
