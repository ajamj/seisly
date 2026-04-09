# Design: Seisly → OpenTect Comprehensive Parity Roadmap (Revised)

**Date:** 2026-04-09 (Revision 2)
**Status:** Under Review — Gate Iteration 2/3
**Author:** AI-assisted brainstorming session
**Scope:** Milestones v1.3.1 through v2.0

---

## Executive Summary

Seisly v1.0.0 is a production-grade seismic interpretation platform with 21 Rust crates. However, ~20 feature gaps remain versus OpenTect 8.1, and **5 critical bugs** threaten data integrity and stability.

This design proposes a **3-track, 8-milestone roadmap** that:
1. **Fixes critical bugs first** (v1.3.1 — isolated, ship immediately)
2. **Delivers user-visible value every milestone** (not just at the end)
3. **Runs independent milestones in parallel** where possible
4. **Defines "minimum viable parity"** — the point where a geophysicist could genuinely replace OpenTect

**Why Seisly over OpenTect?**
- **Rust performance:** Memory-safe, zero-GC, GPU-native vs C++ legacy codebase
- **Modern UX:** VS Code-style IDE with dockable panels vs Qt MDI (OpenTect's 1990s-era UI)
- **Python-first:** Native PyO3 + gRPC + process-isolated worker vs OpenTect's ODBind plugin
- **OpenTect-compatible:** Import OpenTect surveys, use OpenTect algorithms, export to OpenTect formats
- **Free and open source:** GPL-compatible, no vendor lock-in

---

## Use Cases (WHO/WANTS/SO THAT)

### UC-1: Geophysicist Seismic Interpretation
> As a geophysicist, I want to load a SEG-Y volume, navigate through inline/crossline/time slices interactively, pick horizons manually or via auto-tracking, and compute seismic attributes SO THAT I can identify subsurface structures and hydrocarbon indicators.

**Milestones:** v1.3 (visualization), v1.3.1 (stability), v1.4 (velocity), v1.5 (volume processing)

### UC-2: Geologist Stratigraphic Modeling
> As a geologist, I want to define stratigraphic units, correlate well logs with seismic horizons, and build layer sequences SO THAT I can map depositional environments and predict reservoir extent.

**Milestones:** v1.4 (well tie depth), v1.6 (stratigraphy, geobodies)

### UC-3: Reservoir Engineer 4D Analysis
> As a reservoir engineer, I want to monitor time-lapse seismic changes, compute difference volumes, and correlate production data with seismic anomalies SO THAT I can track fluid movement and optimize well placement.

**Milestones:** v1.4 (velocity → depth conversion), v1.5 (volume processing), existing v1.1 4D

### UC-4: OpenTect Migration
> As an OpenTect user, I want to import my existing survey (SEGY + horizons + faults + wells) into Seisly SO THAT I can continue interpretation work without losing data.

**Milestones:** v1.3 (SEG-Y), v1.3.1 (reliable import), v1.4 (CRS, velocity), v2.0 (survey structure)

### UC-5: Data Scientist Plugin Development
> As a data scientist, I want to write a Python plugin that takes seismic data, runs my custom ML model, and returns attribute volumes SO THAT I can integrate my research algorithms into production interpretation workflows.

**Milestones:** v1.3.1 (fix Python bridge), v1.4 (plugin API), v2.0 (plugin ecosystem)

---

## Minimum Viable Parity (MVP)

**v1.4 is the MVP checkpoint.** At v1.4, a geophysicist can:
- Load SEG-Y volumes (reliably, without crashes — v1.3.1 fixes)
- Navigate slices interactively (v1.3)
- Pick horizons manually and auto-track (existing)
- Model faults (existing)
- Compute 20+ seismic attributes (existing, GPU-accelerated)
- Build velocity models and do time-depth conversion (v1.4)
- Use correct CRS coordinates (v1.4)
- Run Python plugins (v1.4)

**This is the "ship it" milestone.** Everything beyond v1.4 is stretch: nice-to-have for power users but not required for a working interpretation tool.

---

## Assumptions

- **Team:** 1–3 developers (AI-assisted)
- **Timeline:** v1.3.1 → v1.4 = 4–6 weeks; v1.5 → v1.6 = 6–8 weeks each; v1.7 → v1.8 = 4–6 weeks each; v1.9 = defer/post-v2.0; v2.0 = 4 weeks
- **Target:** 6–9 months to MVP (v1.4), 12–18 months to v2.0
- **User feedback:** After each milestone, release to a small beta group (3–5 geophysicists) for usability validation
- **OpenTect reference:** Used for algorithmic understanding ONLY (read source, verify output matches). No code copying — GPL licensing risk if Seisly is MIT/Apache-2.0.

---

## Milestones

### Milestone v1.3.1: Critical Stability Fixes (Track 1 — Emergency)
**Theme:** Stop the bleeding — fix crashes and data corruption before building anything new

**User impact:** Without this, users experience crashes during viewport rendering, wrong coordinates in CRS transforms, and non-functional plugins.

**Goals:**
1. **Replace all `.unwrap()` in rendering/data paths** with safe error handling
   - `viewport.rs:124` (sketch_points.last().unwrap()) → `.unwrap_or_default()`
   - `viewport.rs:543–544, 625, 670` (ECS resource unwrap) → `.ok().unwrap_or_else(|| return early with error state)`
   - `well.rs:83–88, trajectory.rs:39` (partial_cmp.unwrap()) → `.unwrap_or(Ordering::Equal)`
2. **CRS transform fails loudly** — return `Err(CrsError::NotImplemented)` instead of identity
3. **SEG-Y parser returns real ranges** — replace hardcoded `(1, 1)` with actual inline/crossline detection
4. **Python bridge type conversion works** — fix `python_to_json()` to actually convert dicts/arrays
5. **FWI gradient math corrected** — implement adjoint state method properly (or mark as non-functional with clear error message)

**Success Criteria:**
- Zero `.unwrap()` panics in non-test code
- CRS transforms return errors for unimplemented projections (visible to user)
- SEG-Y files load with correct inline/crossline ranges
- Python plugins can exchange non-trivial data (test with dict + array roundtrip)
- All changes tested with property-based tests for edge cases

**Dependencies:** None — this is independent of all other work

**Timeline:** 1–2 weeks

---

### Milestone v1.4: Velocity, Color, CRS, Plugins (Track 1 — MVP)
**Theme:** Complete the foundations — make Seisly genuinely usable for a geophysicist

**UCs served:** UC-1 (velocity), UC-2 (well tie), UC-4 (CRS migration), UC-5 (plugins)

**Goals:**

#### Velocity Modeling
- Interval/RMS/average velocity computation
- Velocity function editor (V0+kZ, linear, polynomial)
- Grid-based velocity models
- Time-depth conversion pipeline (horizon-level and volume-level)

#### Color Management
- Editable colormaps with interactive editor (like OpenTect's coltab editing)
- Custom color table import/export (RGB, HSV, colortab format)
- Scene color bar management (position, size, labels)
- Seismic-specific presets: Blue-White-Red, Gray, Rainbow, Wiggle

#### CRS Support
- PROJ-based coordinate reference system integration (replace the stub)
- Position conversion between CRS (geographic ↔ projected ↔ local)
- Survey geometry management
- Position validation (reject out-of-bounds coordinates)

#### Plugin System
- Mature plugin API with Rust trait definitions
- External attributes framework (plugins can register new attribute types)
- Plugin lifecycle: discover → load → configure → execute → results
- Resource limits: CPU time, memory, disk quotas for plugins
- Error surfacing: plugin failures visible to user with actionable messages

#### Performance
- O(N²) smoothing → parallel implementation or FFT-based
- Per-trace SEG-Y allocations → reuse buffers
- UI clone spam → borrow instead of clone in render loop

**TDD Strategy:**
- Velocity: Property-based tests against known velocity function outputs
- Color: Unit tests for each preset colormap, roundtrip import/export tests
- CRS: Test cases from PROJ documentation (known coordinate pairs)
- Plugin: Mock plugin that exercises all API surface, integration test with real Python

**Success Criteria (User-Focused):**
- Geophysicist can build a velocity model from well picks and convert horizons to depth in <5 minutes
- Color editor produces maps that match OpenTect output (visual comparison)
- OpenTect survey imports with correct coordinates (verified against known control points)
- Data scientist can write a 10-line Python plugin that returns a valid attribute volume

**Dependencies:** v1.3.1 (critical fixes), v1.3 (visualization)

**Timeline:** 3–4 weeks

---

### Milestone v1.5: Volume Processing (Track 2, Part A)
**Theme:** Post-processing workflows — transform seismic data into interpretable volumes

**UC served:** UC-3 (4D analysis), UC-1 (attribute volumes)

**Goals:**
- **Processing Chains** — Multi-step UI pipeline (step builder, reorder, preview intermediate results)
- **Smoothing** — Lateral and general volume smoothing (FFT-based, parallel)
- **Math Operations** — Volume arithmetic (add, subtract, multiply, divide, clip, normalize)
- **Body Filling** — Fill volumes between surfaces, horizon-interpolated filling
- **Voxel Connectivity** — Connected component filtering for geobody extraction
- **Statistics** — Volume statistics (mean, RMS, min, max, histogram, percentiles)

**UI Architecture:**
- New dock tab type: `ProcessingChain` (appears when user selects a volume → right-click → "Process")
- Step builder: Vertical list with drag-to-reorder, parameter panels per step
- Preview: Split view showing input vs output for each step
- Progress: Non-blocking overlay with cancel button (reuses existing `tracking_progress` pattern)

**TDD Strategy:**
- Processing chain: Test with 3-step chain on synthetic volume, verify each step output
- Smoothing: Compare against OpenTect smoothing output on same input (within 1% tolerance)
- Voxel connectivity: Test on known connected/disconnected volumes

**Success Criteria (User-Focused):**
- User can build a 3-step processing chain (smooth → math → fill) in <2 minutes
- Processing results are visually correct (no artifacts, no data loss)
- 1GB volume processes in <30 seconds (measured with `criterion` benchmark)

**Dependencies:** v1.4 (velocity models can be processing inputs)

**Timeline:** 4–6 weeks

**Can run in parallel with:** v1.6 (stratigraphy — minimal interaction overlap)

---

### Milestone v1.6: Stratigraphy & Geobody Modeling (Track 2, Part B)
**Theme:** Geological framework and 3D body creation

**UC served:** UC-2 (geologist stratigraphic modeling)

**Goals:**
- **Stratigraphic Units** — Hierarchical column, reference trees, lithology definitions
- **Layer Sequences** — Stratigraphic layer modeling, automated sequence building
- **Geobody Creation** — Marching cubes isosurface extraction, polygon bodies, random position bodies
- **Horizon Operations** — Merge, sort, grid, modify, interpolate, time-depth transform
- **Isopach Maps** — Thickness mapping

**UI Architecture:**
- New dock tab type: `StratigraphicColumn` (hierarchical tree view, like OpenTect's strat tree)
- New dock tab type: `GeobodyEditor` (3D body manipulation with mesh preview)
- Horizon operations: Context menu on horizon items in ProjectExplorer → "Operations" submenu
- Isopach maps: Display as colored surfaces in Viewport (reuse existing horizon rendering)

**Data Model:**
- New SQLite tables: `strat_units`, `strat_layers`, `geobodies`, `isopach_maps`
- `InterpretationState` extended: `stratigraphic_units: Vec<StratUnit>`, `geobodies: Vec<Geobody>`
- Database migration: `0002_stratigraphy.sql` (additive, backward-compatible)

**TDD Strategy:**
- Stratigraphy: Test unit hierarchy, layer sequence generation, attribute computation
- Geobodies: Test marching cubes on known isosurface (sphere, torus → known mesh properties)
- Horizon operations: Test merge/sort/grid on synthetic horizon data

**Success Criteria (User-Focused):**
- Geologist can define a 3-level stratigraphic column and correlate with well logs in <10 minutes
- Geobody extraction from attribute volume produces watertight mesh (no holes, no self-intersections)
- Horizon merge produces a single surface from two overlapping picks without gaps

**Dependencies:** v1.5 (volume processing feeds geobody extraction) — **but can run in parallel with minimal overlap**

**Timeline:** 4–6 weeks

---

### Milestone v1.7: Pre-Stack Processing (Track 2, Part C)
**Theme:** Gather-level seismic analysis

**UC served:** UC-1 (advanced geophysicist needs)

**Goals:**
- **Prestack Data Model** — Gather-based data with offset/azimuth support
- **Stacking** — NMO/DMO stacking, lateral stacking
- **Trim Statics** — Residual statics correction
- **Muting** — Angle and offset-based mutes, mute definition UI (interactive graphical tool)
- **AGC** — Automatic gain control on prestack data
- **Semblance** — Velocity semblance computation and analysis
- **Prestack Viewer** — Dedicated gather viewer with appearance/scaling/shape controls

**UI Architecture:**
- New dock tab type: `PrestackViewer` (displays gathers in offset/azimuth domain)
- Mute definition: Interactive drawing tool on gather view (similar to horizon picking — reuse `Pick` mechanism)
- Prestack viewer: Separate window mode (like OpenTect's dedicated prestack viewer)

**Data Model:**
- New `PrestackGather` type (different from `SeismicVolume` — gather has offset/azimuth dimensions)
- New `TraceProvider` impl for prestack data: `PrestackProvider`
- SQLite: `prestack_volumes` table, `prestack_gathers` table

**TDD Strategy:**
- Prestack data model: Test gather loading, offset/azimuth extraction
- Stacking: Test NMO on synthetic gather (known velocity → flat events after NMO)
- Semblance: Test on synthetic data with known velocity spectrum peak

**Success Criteria (User-Focused):**
- Geophysicist can view a prestack gather, define mutes, and run NMO stack in <15 minutes
- Semblance panel produces interpretable velocity spectra (clear velocity pick)
- Stack pipeline processes ≥50 gathers without errors

**Dependencies:** v1.6 (stratigraphy provides geological context — but loose dependency)

**Timeline:** 4–6 weeks

---

### Milestone v1.8: Advanced Algorithms (Track 3, Part A)
**Theme:** Algorithmic depth

**Goals:**
- **Signal Processing** — FFT pipeline, frequency filtering, spectral tapering, wavelet transforms, Hilbert transform, spectrograms
- **Analysis** — PCA, dip-PCA, curvature, Delaunay triangulation, Hough transform
- **Classification** — Bayesian classification, rock physics modeling, variogram computation

**UI Architecture:**
- New dock tab type: `AlgorithmPanel` (algorithm selector, parameter input, result preview)
- Results: New attribute volumes added to project tree (reuses existing attribute display)

**TDD Strategy:**
- FFT: Test against numpy.fft reference implementation (same input → same output within 1e-10)
- Hilbert: Test on known analytic signal (cosine → Hilbert transform is sine)
- Curvature: Test on known surface (sphere → constant positive curvature)

**Success Criteria:**
- All 11 algorithm categories produce verifiable results
- FFT pipeline processes 1M-sample trace in <5 seconds
- Algorithms match OpenTect reference outputs within 1% tolerance

**Dependencies:** v1.7 (algorithms apply to prestack data too)

**Timeline:** 4–6 weeks

---

### Milestone v1.9: Batch Processing (DEFERRED — Post-v2.0)
**Status:** Deferred. Most OpenTect users run on single workstations. Cluster processing serves a tiny fraction of users. Focus on single-machine performance first.

**Revisit when:** Users request overnight processing jobs or multi-node scaling.

---

### Milestone v2.0: Full Ecosystem (Track 3, Part C)
**Theme:** Complete the package

**Goals:**
- **Survey/Project Management** — Full survey structure (OpenTect directory organization: `Seismics/`, `Surfaces/`, `WellInfo/`, `Geometry/`, `Attribs/`, etc.)
- **GIS Export** — GeoJSON, shapefile, KML export (validated against QGIS/ArcGIS)
- **Documentation** — Complete user manual, API reference, step-by-step tutorials
- **Plugin Ecosystem** — HDF5 import, MATLAB Link, GMT integration, Madagascar plugins

**UI Architecture:**
- New dock tab type: `SurveyManager` (survey creation, import/export, directory browsing)
- Settings → Import/Export panel for GIS formats

**TDD Strategy:**
- Survey: Test import of existing OpenTect survey → verify all objects loaded correctly
- GIS export: Export known surface → load in QGIS → verify geometry matches

**Success Criteria (User-Focused):**
- OpenTect user can export survey from OpenTect, import into Seisly, and continue working without data loss
- GIS export produces files readable by QGIS/ArcGIS without errors
- New user can complete a full interpretation workflow (load → pick → attribute → export) in <30 minutes using only the user manual

**Dependencies:** v1.8 (all features must be stable before final ecosystem packaging)

**Timeline:** 3–4 weeks

---

## Parallel Execution Plan

```
v1.3 (current) ──────────────────────────────────────────────►
                    │
v1.3.1 (critical) ──┘
                    │
v1.4 (MVP) ─────────┘
                    │
                    ├──► v1.5 (volume processing) ──┐
                    │                                ├──► v1.7 (prestack) ──► v1.8 (algorithms) ──► v2.0
                    └──► v1.6 (stratigraphy) ───────┘
```

v1.5 and v1.6 can run in parallel — their interaction surface is limited (volume processing feeds geobody extraction, but stratigraphic units are independent).

---

## Architecture Decisions (Revised)

### A1: Maintain Rust + egui + wgpu Stack
**Rationale:** Strong existing investment. egui_dock provides VS Code-style IDE layout. wgpu provides cross-platform GPU compute.

### A2: Fix Debt Before New Features — ISOLATED
**Rationale:** v1.3.1 is its own milestone. Ship it before starting v1.4. Building velocity models on broken CRS transforms is wasted effort.

### A3: MVP at v1.4
**Rationale:** v1.4 is the first milestone where a geophysicist can do a complete interpretation workflow. Ship it, get user feedback, then iterate.

### A4: Algorithmic Reference Only (No Code Copying)
**Rationale:** OpenTect is GPL. Seisly is MIT/Apache-2.0. Reading OpenTect for algorithmic understanding is fine; copying implementation creates licensing risk. All implementations must be original Rust code.

### A5: Plugin API Designed Up Front
**Rationale:** Plugin API surface (Rust traits, Python FFI boundary, data exchange format, security model) gets a separate design document before v1.4 implementation. Getting this wrong means breaking changes later.

### A6: GPR Deferred
**Rationale:** Ground Penetrating Radar uses fundamentally different data formats and processing chains. Spin off as separate project if needed.

### A7: Server Mode Depreciated
**Rationale:** `seisly_ai` and `seisly_server` workspace members are dead weight. Remove from workspace. Python gRPC client (`ai_client.rs`) can be re-enabled when needed.

### A8: UI Expansion Strategy
**Rationale:** Each new major feature (Prestack, Stratigraphy, Processing Chains) gets its own dock tab type. The activity bar grows to accommodate new categories. Existing 8-tab structure extends to ~15 tabs by v2.0. If egui_dock proves insufficient, custom multi-window support will be added.

---

## Risk Assessment (Revised)

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Scope creep (too many milestones) | **Medium** (reduced by MVP checkpoint) | High | v1.4 is "ship it" milestone; v1.5+ are stretch |
| OpenTect reference code too complex to port | Medium | Medium | Algorithms only, not implementation; simpler approaches where possible |
| egui limitations for complex UI | Medium | Medium | Custom dock tab types per feature; multi-window fallback if egui_dock insufficient |
| wgpu shader complexity | Low | Low | Existing seismic.wgsl works; extend incrementally |
| Performance not matching OpenTect | Medium | Medium | `criterion` benchmarks per crate; GPU compute where CPU is bottleneck |
| Technical debt compounds during expansion | High | High | v1.3.1 fixes all critical debt first; code review enforced for all new features |
| Licensing risk (OpenTect GPL) | **Medium** | **High** | A4: Algorithmic reference only; no code copying |
| Single-developer bottleneck | **High** | **High** | Parallel v1.5/v1.6 execution; AI-assisted development |

---

## Success Metrics (Revised — User-Focused)

### MVP Checkpoint (v1.4)
- **Time to first interpretation:** New user can load SEG-Y, pick a horizon, compute an attribute, and export results in <30 minutes
- **Zero data loss:** SEG-Y import preserves all trace data and geometry (verified against reference)
- **Zero crashes:** No `.unwrap()` panics in production code (enforced by CI)
- **Velocity accuracy:** Velocity model produces depth conversions within 1% of known reference values

### Full Parity (v2.0)
- **Feature coverage:** ≥90% of OpenTect core features (measured by feature inventory checklist)
- **Performance:** ≤2x OpenTect execution time for equivalent operations (measured with `criterion` benchmarks)
- **Test coverage:** ≥70% across all crates (enforced by CI)
- **User adoption:** 3+ geophysicists use Seisly as primary interpretation tool for ≥1 month

### Failure Criteria (Kill Switch)
- **If v1.4 takes >12 weeks:** Cut scope to velocity modeling + CRS only; defer color editing and plugins to v1.5
- **If egui proves unsuitable for Flat View (v1.5):** Switch to multi-window approach or iced; do not rewrite existing UI
- **If OpenTect algorithm porting takes >4 weeks per algorithm:** Skip that algorithm; focus on top 10 attributes only
- **If plugin API design takes >2 weeks:** Ship basic Python bridge first; iterate on API design in parallel

---

## TDD Strategy Summary

Every milestone must have:
1. **Test specifications** written before implementation (RED phase)
2. **Edge case enumeration** per component (empty input, max input, malformed input)
3. **Mock infrastructure** — `InMemoryProvider` for seismic data, `MockTraceProvider` for trace access
4. **Property-based tests** for numerical algorithms (velocity, CRS, FFT)
5. **Integration test helpers** — `TestProject` fixture that creates a temporary project with sample data

Benchmarking:
- `criterion` benchmarks added to every crate with performance-sensitive code
- Benchmarks run in CI on every PR (compare against baseline, flag >10% regression)
- OpenTect reference benchmarks (same input, same output, compare execution time)

---

## Deferred Ideas (Categorized by Persona)

| Idea | Persona | Potential Milestone |
|------|---------|---------------------|
| Cloud-native distributed processing (S3) | Reservoir Engineer | Post-v2.0 |
| Real-time seismic streaming (live field) | Geophysicist | Post-v2.0 |
| AI-powered auto-interpretation (beyond CNN/U-Net) | Data Scientist | Post-v2.0 |
| Collaborative interpretation (multi-user) | All | Post-v2.0 |
| AR/VR visualization | Geophysicist | Post-v2.0 |
| Full Waveform Inversion production | Geophysicist | Post-v2.0 |
| Batch/cluster processing (v1.9) | Reservoir Engineer | Deferred |
| GPR support | Geophysicist (GPR specialists) | Separate project |

---

## Canonical References

### Project Context
- `.planning/codebase/CONCERNS.md` — 5 critical concerns, 17+ stubs, performance bottlenecks
- `.planning/codebase/ARCHITECTURE.md` — System architecture and crate responsibilities
- `.planning/codebase/STACK.md` — Technology stack inventory
- `.planning/codebase/STRUCTURE.md` — Directory and module structure
- `.planning/codebase/TESTING.md` — Testing patterns and coverage gaps
- `.planning/codebase/CONVENTIONS.md` — Coding conventions and patterns
- `.planning/codebase/INTEGRATIONS.md` — External integrations
- `.planning/ROADMAP.md` — Current project roadmap

### Current Plans
- `.planning/phases/v1.3-seismic-visualization-PLAN.md` — Current v1.3 visualization plan
- `.planning/phases/v1.3-q1-segy-optimization-PLAN.md` — Current v1.3 SEG-Y optimization plan

### OpenTect Reference (Algorithmic Only — No Code Copying)
- `references/OpendTect/src/Attributes/` — 29 seismic attribute implementations
- `references/OpendTect/src/MPEEngine/` — Horizon auto-tracking engine
- `references/OpendTect/src/visSurvey/` — Visualization architecture reference
- `references/OpendTect/src/VolumeProcessing/` — Volume processing chains
- `references/OpendTect/src/Strat/` — Stratigraphic modeling
- `references/OpendTect/src/PreStackProcessing/` — Prestack algorithms
- `references/OpendTect/src/Algo/` — Core algorithms (FFT, curvature, etc.)

### Design Decisions
- `agents/product-manager.md` — Product Manager review agent definition
- `agents/architect.md` — Architect review agent definition
- `agents/designer.md` — Designer review agent definition
- `agents/security-design.md` — Security Design review agent definition
- `agents/cto.md` — CTO review agent definition
- `docs/plans/opentect-parity-roadmap-design.md` — This document

---

*Revision 2 — 2026-04-09*
*Changes from Revision 1: Added use cases (UC-1 through UC-5), isolated v1.3.1 critical fixes, defined MVP checkpoint at v1.4, added timeline/assumptions, added failure criteria, added parallel execution plan, added user-focused success metrics, added TDD strategy, added plugin security requirements, added UI architecture plan, deferred v1.9, deprecated server mode, clarified GPL licensing constraint*
