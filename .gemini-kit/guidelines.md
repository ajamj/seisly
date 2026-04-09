# Development Guidelines — Seisly

## Code Style

- **Indentation:** 4 spaces (enforced by `cargo fmt --all`)
- **Edition:** Rust 2021
- **Naming:** `snake_case` for functions/variables, `PascalCase` for types/traits, `SCREAMING_SNAKE_CASE` for constants
- **Module Organization:** `mod.rs` for module declarations, one file per logical component
- **Line Length:** No hard limit, but keep under ~120 chars when reasonable

## Error Handling

- **Library Errors:** Use `thiserror` derive macro for custom error enums
- **Application Errors:** Use `anyhow` for top-level error propagation
- **Result Types:** Return `Result<T, SeislyError>` from public APIs, `anyhow::Result` from binaries
- **Panics:** Avoid `.unwrap()` in production code — use `.ok_or_else()`, `.and_then()`, or early returns

## Entity Identification

- **IDs:** UUID v4 for all domain entities (wells, horizons, faults, surfaces, etc.)
- **Generation:** `uuid::Uuid::new_v4()` at creation time, serialized via `serde`
- **CRS Codes:** EPSG codes as strings (e.g., "EPSG:32648")

## Testing Practices

- **Location:** Inline `#[cfg(test)]` modules in source files — **not** separate `tests/` directory
- **Mocking:** Use `InMemoryProvider` trait implementation for seismic data mocking
- **Test Data:** Synthetic SEG-Y buffers generated via test fixtures
- **NaN Safety:** Explicitly test `total_cmp()` behavior with NaN inputs
- **GPU Tests:** Pipeline init tests verify shader compilation and linkage
- **Benchmarks:** Use `criterion` in `benches/` directory

## Git Conventions

- **Commits:** Atomic per task — each commit should be independently testable
- **Messages:** Clear, concise, focused on "why" over "what"
- **Branches:** Feature branches for implementation work
- **Merging:** Rebase onto main, resolve conflicts before PR

## Documentation

- **Doc Comments:** Standard Rust `///` and `//!` doc comments
- **Safety:** `#[allow(unsafe_code)]` with documented `/// # Safety` invariant on every unsafe block
- **API Docs:** Generated at `api/` via `cargo doc --open`
- **User Manual:** `docs/user-manual/` (mdBook)

## Unsafe Code

- **Policy:** Minimize unsafe code — prefer safe wrappers
- **Annotation:** Every `unsafe` block requires `#[allow(unsafe_code)]` and `/// # Safety` documentation
- **Example:** `SafeMmap::as_slice()` documents that callers must not access beyond mapped region
- **Thread Safety:** Use `Arc<Mutex<T>>` or `Send + Sync` bounds for cross-thread data sharing

## Float & Numeric Safety

- **Comparisons:** Use `total_cmp()` for float ordering — **never** `partial_cmp().unwrap()` (NaN will panic)
- **Overflow:** Use `checked_mul`, `checked_add` for stride calculations in SEG-Y indexing
- **Division:** Guard against zero divisors in attribute computation

## Performance

- **Large Arrays:** Route through shared memory (SHM) when >1MB threshold
- **Parallelism:** Use `rayon` for CPU-bound parallel iteration, `tokio` for async I/O
- **Caching:** LRU brick cache (moka) for out-of-core seismic volume access
- **GPU:** Upload textures only on change, cache in `CallbackResources`
