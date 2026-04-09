# Project Context (Maintained by Orchestrator)

## Tooling
- Package manager: cargo (Rust workspace)
- Test runner: cargo test (inline #[cfg(test)] modules)
- Linter: cargo clippy -- -W clippy::all
- Build: cargo check/build
- Coverage: cargo tarpaulin --fail-under 70 (blocking gate per .coverage-thresholds.json)

## Completed Work Units
| WU | Title | Key Files | Services Created |
|----|-------|-----------|-----------------|
| v1.3-01 | Seismic Visualization Core | colormaps.rs, seismic.wgsl, seismic_renderer.rs, lib.rs | SliceType enum, Rainbow/BlueWhiteRed colormaps |
| v1.3-02 | Viewport Integration | viewport.rs | 3-axis slicing UI, unwrap fixes, SliceType dispatch |
| v1.3-04 | Float Safety & SafeMmap | well.rs, trajectory.rs, safe_mmap.rs | total_cmp() replacements, SafeMmap docs |
| v1.3-04b | CRS Explicit Errors | transformer.rs | TransformError::NotImplemented variant |
| v1.3-03 | SEG-Y Performance | index.rs, mmap.rs, parser.rs | SegyIndex schema, scan_trace_headers, get_trace_offset |

## Established Patterns
- **NaN safety**: total_cmp() replaces partial_cmp().unwrap() everywhere
- **SafeMmap**: #[allow(unsafe_code)] + /// # Safety docs on every unsafe block
- **SliceType enum**: Shared across seisly_render and seisly_app for slice dispatch
- **Colormap presets**: Seismic, Viridis, Magma, Gray, Rainbow, BlueWhiteRed
- **Cross-correlation throw**: trace-based fault throw calculation in seisly_compute

## Active Services
- SEG-Y I/O: memory-mapped with persistent sidecar indexing (bincode)
- Seismic rendering: wgpu pipelines with uniform-based wiggle scaling
- Viewport: egui_dock with three-axis slicing, dynamic slider ranges
