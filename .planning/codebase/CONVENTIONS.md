# Coding Conventions

**Analysis Date:** 2026-03-31

## Naming Patterns

**Files:**
- Snake case: `project.rs`, `lib.rs`, `main.rs`, `ai_client.rs`.

**Functions:**
- Snake case: `pub fn load(project_path: &Path) -> Result<Self, ProjectError>`.
- Test functions: `test_` prefix (e.g., `test_project_creation`).

**Variables:**
- Snake case: `project_path`, `manifest_path`, `content`.

**Types:**
- PascalCase: `ProjectManifest`, `SeismicVolumeEntry`, `ProjectError`.
- Type Aliases: `EntityId = Uuid`.

## Code Style

**Formatting:**
- `rustfmt` - Standard Rust formatting.
- Check: `cargo fmt --all -- --check` (enforced in CI).
- Apply: `cargo fmt --all`.

**Linting:**
- `clippy` - Enforced in CI with `cargo clippy --workspace --all-targets -- -W clippy::all`.

## Import Organization

**Order:**
1. Standard library (`std::...`).
2. External crates (`serde`, `uuid`, etc.).
3. Internal workspace crates (`crate::...`, `seisly_core::...`).

**Path Aliases:**
- Standard Rust module paths (no custom `tsconfig`-like aliases detected).

## Error Handling

**Patterns:**
- Libraries: `thiserror` for custom error enums with `#[error]` and `#[from]`.
- Binaries/Application: `anyhow` for top-level error handling (e.g., `anyhow::Result<()>`).
- Crate-specific Result types: `pub type Result<T> = std::result::Result<T, Error>`.

## Logging

**Framework:** `tracing` and `tracing-subscriber` (found in `Cargo.toml`, though usage is minimal in current code).

**Patterns:**
- No extensive logging pattern observed in current source files.

## Comments

**When to Comment:**
- Module-level descriptions: `//! Module description`.
- Public item documentation: `/// Item description`.

**JSDoc/TSDoc:**
- Not applicable (Rustdoc is used).

## Function Design

**Size:** Generally small, focused functions (e.g., `load`, `save`, `create`).

**Parameters:** Prefer explicit types (e.g., `&Path`, `String`, `u32`).

**Return Values:** Use `Result<T, E>` for operations that can fail.

## Module Design

**Exports:** explicit `pub` and `pub(crate)` visibility.

**Barrel Files:** `mod.rs` or `lib.rs` for module re-exports (e.g., `crates/seisly_core/src/lib.rs`).

---

*Convention analysis: 2026-03-31*
