# Technical Concerns & Improvement Opportunities

> Generated: 2026-04-09
> Scope: Full codebase analysis across 20 crates, 161 Rust source files

---

## Critical Concerns

### CRS Transformer Returns Identity Transform (Silent Data Corruption Risk)
- **Location:** `D:\GRC-Ajam\seisly\crates\seisly_crs\src\transformer.rs:44-52`
- **Issue:** `transform_points()` returns points **unchanged** with a comment "Placeholder implementation". The `proj` crate is a workspace dependency but is never actually used for coordinate transformation. Any code relying on CRS transforms will produce silently wrong coordinates.
- **Impact:** Any well trajectories, survey geometry, or interpreted picks that require CRS conversion will have wrong coordinates without any warning to the user.

### FWI Gradient Computation Uses Simplified/Incorrect Math
- **Location:** `D:\GRC-Ajam\seisly\crates\seisly_fwi\src\acoustic.rs:140-155`
- **Issue:** The gradient computation multiplies `wavefield_fwd * wavefield_fwd` (same wavefield squared) instead of `u_fwd * u_adj` (forward times adjoint). The adjoint wavefield is never computed (marked `// TODO: Implement adjoint`). This means FWI velocity updates are mathematically incorrect.
- **Impact:** If a user runs FWI, the velocity model will be updated with wrong gradients, producing meaningless results.

### Python Plugin Bridge Has Non-Functional Type Conversion
- **Location:** `D:\GRC-Ajam\seisly\crates\seisly_plugin\src\python.rs:47-57`
- **Issue:** `python_to_json()` is labeled "Hacky placeholder" and returns an empty JSON object for ALL Python dict inputs. `json_to_python()` returns `"{}"` for arrays and objects. This means any plugin that takes arguments or returns complex data will silently lose all data.
- **Impact:** Plugin system is non-functional for any real use case beyond nullary commands.

### Unwrap on Empty Collections in Viewport Widget
- **Location:** `D:\GRC-Ajam\seisly\crates\seisly_app\src\widgets\viewport.rs:124-126`
- **Issue:** `self.sketch_points.last().unwrap()[0/1/2]` will panic if `sketch_points` is empty. This is in UI rendering code that runs every frame.
- **Impact:** Crash when rendering viewport with no sketch points.

### Unwrap on Resource Access in Viewport
- **Location:** `D:\GRC-Ajam\seisly\crates\seisly_app\src\widgets\viewport.rs:543-544, 625, 670`
- **Issue:** Multiple `.unwrap()` calls on `resources.get::<SeismicRenderer>()`, `resources.get::<ColormapManager>()`, and `resources.get_mut::<SeismicResources>()`. If the ECS resources are not properly initialized, these will panic at runtime.
- **Impact:** Crash during viewport rendering if resource initialization fails or is delayed.

---

## Technical Debt

### Placeholder/Stub Implementations (17 confirmed)

| File | Component | Status |
|------|-----------|--------|
| `seisly_crs/transformer.rs` | CRS coordinate transformation | Returns identity (no-op) |
| `seisly_io/segy/parser.rs:30-36` | SEG-Y inline/crossline range detection | Hardcoded `(1, 1)` |
| `seisly_tracking/fault_guided.rs` | Fault-guided horizon tracking | Returns seed point only |
| `seisly_fwi/misfit.rs:48` | Travel-time sensitivity kernel | Returns zero gradient |
| `seisly_fwi/acoustic.rs:148` | Adjoint state method | Not implemented |
| `seisly_ml_deep/fault_detection.rs:93,103` | Connectivity analysis, throw calculation | Not implemented |
| `seisly_ml_deep/training_dl.rs:141` | Data augmentation | Returns clone (no augmentation) |
| `seisly_plugin/python.rs:47` | Python dict-to-JSON conversion | Returns empty object |
| `seisly_plugin/manager.rs` | Discovered plugin loading | Uses `PlaceholderPlugin` when Python feature disabled |
| `seisly_compute/clipping.rs:108` | Segment-to-polyline connection | Returns individual segments |
| `seisly_compute/throw.rs:19` | Barycentric interpolation for Z | Not implemented |
| `seisly_app/ai_client.rs` | gRPC AI detection client | Compiles but untested |
| `seisly_app/time_lapse_panel.rs:56` | 4D trace data loading | Placeholder logic |
| `seisly_py_worker/main.rs:188` | Complex type conversion | Falls back to string |

### FaultGuidedTracker Returns Seed Point Only
- **Location:** `seisly_tracking/src/fault_guided.rs:15-17`
- **Issue:** `track()` ignores the `TraceProvider` entirely and returns `vec![seed]`. The entire fault-guided tracking workflow is non-functional.
- **Impact:** Any UI entry point for fault-guided horizon tracking produces meaningless single-point results.

### Large Files Needing Decomposition

| File | Lines | Concern |
|------|-------|---------|
| `seisly_app/src/app.rs` | ~914 | Monolithic app struct with 30+ fields, all UI logic in one file |
| `seisly_app/src/widgets/viewport.rs` | ~670 | Rendering, input handling, sketch mode, GPU resource management all mixed |
| `seisly_fwi/src/acoustic.rs` | ~250 | Wave equation, FWI inversion, smoothing, and tests in single file |

### Unwrap/Expect in Non-Test Production Code

Beyond the critical items above, these locations use `.unwrap()` without defensive handling:

- `seisly_core/domain/well.rs:83-88, 117-122` — `partial_cmp().unwrap()` on float comparisons (NaN will panic)
- `seisly_core/domain/trajectory.rs:39` — `partial_cmp().unwrap()` on depth sorting
- `seisly_app/src/main.rs:641` — `path.file_name().unwrap()` in busy message formatting
- `seisly_render/seismic_renderer.rs:177` — `.unwrap()` on GPU device request in test (acceptable for test, but pattern leaks)
- `seisly_tracking/multi_horizon.rs:34, 106` — `unwrap_or_else` and `.unwrap()` on ML model initialization
- `seisly_attributes/src/frequency.rs:53` — `partial_cmp().unwrap()` in FFT frequency analysis (NaN risk)
- `seisly_attributes_gpu/src/compute.rs:252, 258` — `.unwrap()` on channel send/receive in GPU compute path

### CLI List Command Fully Non-Functional
- **Location:** `seisly_cli/src/commands/list.rs:16-28`
- **Issue:** `list` subcommand prints "(not yet implemented - requires SQLite queries)" for wells, surfaces, logs, and datasets. The only output is the project manifest name and version.
- **Impact:** Users of the `sf` CLI have no way to query project contents from the command line.

### LAS 3.0 Parser Rejects Version 3.0 Files
- **Location:** `seisly_io/src/las/parser.rs:53`
- **Issue:** The parser returns an error "LAS 3.0 not yet implemented" when encountering version 3.0 headers, despite the existence of a separate `LasV3Reader` in `seisly_io/src/las/v3.rs`. The parser dispatch logic does not route to the v3 reader.
- **Impact:** LAS 3.0 files cannot be imported through the standard LAS parser path.

### Zero-Copy Bridge Has Unsound Lifetime Assumption
- **Location:** `seisly_plugin/src/bridge.rs:19`
- **Issue:** `share_with_python()` uses `PyArrayDyn::borrow_from_array_bound()` with a reference to Rust-owned data, but the safety comment says "The caller must ensure the data slice outlives the NumPy array." There is no compile-time enforcement — if the Rust Vec is dropped while Python still holds the array, this is undefined behavior.
- **Impact:** Potential memory corruption if plugin code retains references to transferred seismic data after the source vector is freed.

### Dead Code Markers
- Extensive `#[allow(dead_code)]` annotations across `seisly_app/src/app.rs` (VisualSettings fields, name field, various state)
- `AiClient` struct in `seisly_app/src/ai_client.rs` is entirely `#[allow(dead_code)]` — compiled but unused
- `seisly_server` referenced in Cargo.toml comments but no `seisly_server` crate exists in workspace

---

## Performance Bottlenecks

### O(N*M*K) Nested Loops in FWI
- **Location:** `seisly_fwi/src/acoustic.rs:38-56` — Forward modeling uses triple nested loops (time × z × x) with no SIMD or parallelization. For a 100×100 grid over 1000 timesteps, this is 10M iterations in Rust single-threaded.
- **Hot path:** `apply_absorbing_boundary()` also iterates the full grid per timestep.

### O(N^2) Per-Pixel Smoothing
- **Location:** `seisly_fwi/src/acoustic.rs:172-187` — Gaussian smoothing uses nested loops over sigma window. Called every velocity update iteration.

### L2 Misfit Linear Scan on `.as_slice().unwrap()[i]`
- **Location:** `seisly_fwi/src/misfit.rs:16-29` — Converts ndarray to slice via `.unwrap()` then iterates element-by-element. Should use ndarray's native iterator or zip for better cache behavior.

### SEG-Y Trace Iterator Creates Full Allocation Per Trace
- **Location:** `seisly_io/src/segy/reader.rs:68-86` — `read_trace()` returns `Arc<Vec<f32>>`, allocating a new Vec for every trace access. For interactive slicing across hundreds of traces, this generates significant GC pressure.

### SeismicVolume Slice Extraction Allocates Full Slices
- **Location:** `seisly_compute/src/seismic.rs:21-35, 41-55, 61-79` — `get_inline()`, `get_crossline()`, `get_time_slice()` all allocate `Vec<f32>` for the entire slice. A 500×500 time-slice at 512 samples = 250K f32s per call, with no caching between calls.

### Clone Spams in App UI Loop
- **Location:** `seisly_app/src/app.rs` — Multiple `.clone()` calls per UI frame on `Theme`, `PathBuf`, `mpsc::Sender`, and `egui::Context`. While individual clones are cheap, the cumulative effect in a 60fps render loop adds up.

---

## Architecture Smells

### `seisly_app` God Object
The `SeislyApp` struct has 30+ fields spanning: interpretation state, GPU resources, plugin management, theme management, project state, import state, UI layout state, crossplot widget, viewport widget, velocity panel, well panel, QI panel, time-lapse panel, diagnostics, settings, and history management. This violates single responsibility and makes the crate difficult to test.

### Circular-ish Dependency Pattern
`seisly_compute` depends on `seisly_core` and `seisly_io`, while `seisly_io` re-exports types from `seisly_compute::seismic`. The `SeismicVolume` struct lives in `seisly_compute` but its `TraceProvider` trait is re-exported from `seisly_core`. This creates conceptual coupling across three crates for what is essentially one domain concept.

### Plugin System Incomplete Abstraction
The `PluginManager` uses `PlaceholderPlugin` when the `python` feature is disabled, but the discovery logic silently accepts plugins it cannot execute. Users see plugins listed but get "Plugin not yet fully loaded" errors on execution — no clear error path from discovery to execution failure.

### ML Crates Fragmented Without Clear Boundaries
Three separate ML crates exist: `seisly_ml` (HorizonCNN, training), `seisly_ml_deep` (UNet, fault detection, deep learning trainer), and `seisly_ai` (Python-only AI server). The boundary between `seisly_ml` and `seisly_ml_deep` is unclear — both handle horizon detection with different models.

### `seisly_server` Referenced but Missing
Workspace Cargo.toml mentions `axum`, `tower`, `tonic`, `prost` dependencies "for seisly_server" but no `seisly_server` crate exists in the workspace `members` list. These are dead dependencies that increase compilation time.

---

## UI/UX Gaps

### No Undo/Redo for Interpretation Picks
The `HistoryManager` exists in `seisly_app/src/interpretation/history.rs` but is not wired into the viewport sketch/pick workflow. Users cannot undo horizon picks or fault interpretations.

### Search Panel Not Implemented
The Explorer sidebar has a "Search" tab that displays "Search implementation coming soon..." — no actual search functionality.

### 4D/Time-Lapse Panel Has Placeholder Logic
The time-lapse difference computation in `time_lapse_panel.rs` uses placeholder logic that doesn't actually load trace data from volumes.

### Error Recovery UX
The crash reporter in `main.rs` shows a dialog but the panic hook still calls `sentry::integrations::panic::panic_handler` after setting a custom message — potential double-reporting.

### Diagnostics Panel Delegates to Bottom Panel
The Diagnostics activity bar tab shows "Diagnostics (Logs) are shown in the bottom panel" rather than actually rendering anything, creating a dead-end navigation path.

---

## Security Considerations

### No Input Validation on SEG-Y File Paths
`SegyReader::open()` takes any path without validation. On Windows, long paths or UNC paths could cause unexpected behavior.

### Memory-Mapped File Safety
The `SafeMmap` wrapper (`seisly_core/src/io/safe_mmap.rs`) is well-implemented with bounds checking. However, `as_slice()` method is `unsafe` and publicly exposed — any caller can bypass safety guarantees. The `SafeMmapArc` wrapper provides no additional safety over the base type.

### Plugin Execution Isolation
Python plugins execute via `subprocess` (based on `seisly_plugin/src/python_plugin.rs`) but there is no sandboxing, resource limits, or timeout enforcement. A misbehaving plugin could hang or consume unbounded resources.

### No Authentication or Authorization
The application is entirely offline/single-user. The planned server mode (referenced by axum/tonic dependencies) has no implementation, so there is no RBAC, no session management, and no data encryption at rest.

### gRPC AI Client Has No TLS Configuration
`AiClient::connect()` in `ai_client.rs` connects to a gRPC endpoint without any TLS configuration, meaning any future AI analysis would transmit seismic data over plaintext connections.

---

## Scalability Limits

### In-Memory Seismic Volume Design
The `InMemoryProvider` stores all data as `Vec<f32>` with no chunking, tiling, or out-of-core strategy. The 501×501×512 test volume in `app.rs` already allocates ~500MB. Production volumes (2000×2000×1000) would require 16GB+ as a single allocation.

### No Brick/Chunk Caching Strategy
While `BrickCache` exists in `seisly_io/src/cache.rs`, the `SegyReader` only uses it for individual trace caching. There is no spatial locality prefetching or LRU eviction, meaning random-access interpretation patterns will cause excessive I/O.

### BFS Horizon Tracking Visits Every Trace
`track_event()` in `seisly_compute/src/tracking.rs` uses BFS with a `HashSet` for visited tracking. For a 2000×2000 survey, the visited set alone could hold 4M entries, and each trace lookup is a separate I/O operation.

### GPU Attribute Computer Limited Scope
Only RMS, Mean, and Energy attributes are GPU-accelerated. Frequency attributes (instantaneous frequency, peak frequency) and geometric attributes (dip, azimuth, coherence) still run on CPU, creating a performance cliff for advanced QI workflows.

---

## Improvement Opportunities

### Low-Hanging Fruit

1. **Replace `.unwrap()` with proper error handling** in `viewport.rs:124-126` and `well.rs:83-88` — these are crash risks in user-facing code.
2. **Implement actual CRS transformation** using the already-included `proj` crate — currently the `seisly_crs` crate is a no-op.
3. **Wire up the Python dict-to-JSON bridge** — this unblocks the entire plugin ecosystem.
4. **Add `#[must_use]` to Result-returning functions** that currently have their results silently ignored.
5. **Remove dead `seisly_server` dependencies** (axum, tower, utoipa, tonic-build) if server mode is not actively being developed.

### Refactoring Opportunities

6. **Extract `SeislyApp` into smaller components** — at minimum, separate the UI state management from the business logic (interpretation, volume management, GPU compute).
7. **Introduce a `seisly_geophysics` crate** — consolidate `seisly_fwi`, `seisly_attributes`, and `seisly_compute`'s geophysical algorithms into a single well-organized crate.
8. **Define a clear trait-based plugin API** — replace the current `Plugin` trait with a versioned API that supports capability negotiation and proper error types.
9. **Add spatial indexing for interpretation data** — horizons, faults, and surfaces could benefit from R-tree or similar spatial index for fast hit-testing in the viewport.

### Pattern Extraction

10. **Standardize error types across crates** — each crate defines its own `Error` enum. A shared `seisly_error` crate or workspace-level error type would simplify error propagation.
11. **Create a `TraceProvider` adapter library** — common operations (slicing, resampling, clipping) could be implemented as generic adapters over any `TraceProvider`, reducing code duplication across `seisly_compute`.
12. **Establish a benchmarking harness** — the single benchmark in `seisly_attributes_gpu/benches/gpu_benchmark.rs` should be expanded to cover all compute-heavy paths.

---

## OpenTect Feature Gaps

Based on comparison against OpenTect's source tree structure (`references/OpendTect/src/`), the following major feature areas are missing from Seisly:

### High Priority (Core Interpretation Workflow)
| OpenTect Module | Seisly Status | Impact |
|-----------------|---------------|--------|
| `uiViewer2D` — Dedicated 2D seismic viewer | Partial (viewport.rs) | 2D line viewing, wiggle trace display |
| `uiViewer3D` — 3D volume rendering | Not started | Ray-casting, iso-surface rendering |
| `Well` — Well data management | Partial (basic LAS import) | Well-to-seismic correlation, well tops |
| `WellAttrib` — Well attribute extraction | Not started | Synthetic seismograms from well logs |
| `uiSeis` — Seismic visualization UI | Partial | Polarity control, gain/AGC |
| `Geometry` — Survey geometry management | Not started | Bin grid definition, fold maps |

### Medium Priority (Advanced Analysis)
| OpenTect Module | Seisly Status | Impact |
|-----------------|---------------|--------|
| `AttributeEngine` — Multi-volume attribute computation | Partial (GPU for 3 attrs) | Dozens of seismic attributes |
| `Strat` — Stratigraphic analysis | Not started | Sequence boundaries, chronostrat |
| `Velocity` — Velocity modeling | Partial (linear model only) | Time-depth conversion, interval velocity |
| `PreStackProcessing` — Pre-stack data handling | Not started | Gathers, AVA analysis |
| `uiFlatView` — Flattened seismic views | Not started | Flattening on horizons |
| `MMProc` — Multi-module processing | Not started | Batch attribute workflows |

### Lower Priority (Enterprise/Specialized)
| OpenTect Module | Seisly Status | Impact |
|-----------------|---------------|--------|
| `EMAttrib` — Electromagnetic attributes | Not started | CSEM integration |
| `EarthModel` — Earth model building | Not started | Geomodel export |
| `NLA` — Non-linear attributes | Partial | Higher-order statistics |
| `uiSysAdm` — System administration | Not started | User management, licensing |
| `Batch` — Batch processing | Not started | Headless/automated workflows |
| `Network` — Distributed computing | Not started | Cluster processing |
