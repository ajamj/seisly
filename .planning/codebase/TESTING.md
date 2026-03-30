# Testing Patterns

**Analysis Date:** 2026-03-31

## Test Framework

**Runner:**
- `cargo test` (standard Rust test runner).
- Config: `Cargo.toml`.

**Assertion Library:**
- Standard Rust assertions: `assert!`, `assert_eq!`, `assert_ne!`.

**Run Commands:**
```bash
cargo test --workspace      # Run all tests
cargo test -p <crate_name>  # Run tests for specific crate
cargo bench                 # Run benchmarks
```

## Test File Organization

**Location:**
- Unit tests: In-module (`#[cfg(test)] mod tests`).
- Integration tests: Root `tests/` directory (e.g., `tests/phase1_integration_test.rs`).

**Naming:**
- Integration test files: `<name>_test.rs`.
- Test functions: `test_<functionality>`.

**Structure:**
```
[project-root]/
├── tests/              # Integration tests
│   ├── phase1_integration_test.rs
│   └── ...
└── crates/
    └── [crate]/
        └── src/
            └── [file].rs # Unit tests in mod tests { ... }
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // Setup
        // Execute
        // Assert
    }
}
```

**Patterns:**
- Setup pattern: Create temporary files/directories (e.g., `tempfile::TempDir::new()`).
- Teardown pattern: RAII handles cleanup (e.g., `TempDir` drop deletes the directory).
- Assertion pattern: Use `unwrap()` for internal test logic and `assert!` for results.

## Mocking

**Framework:** None explicitly used in current code.

**Patterns:**
- No explicit mock frameworks (like `mockall`) detected.
- Instead, uses synthetic data generators (e.g., `SyntheticTrainer`).

**What to Mock:**
- Not explicitly defined.

**What NOT to Mock:**
- Core domain logic and math algorithms (prefer real execution).

## Fixtures and Factories

**Test Data:**
```rust
let mut trainer = SyntheticTrainer::new(42);
let (seismic, labels) = trainer.generate_batch(10, 64);
```

**Location:**
- `SyntheticTrainer`: `crates/seisly_ml/src/trainer.rs` (likely).

## Coverage

**Requirements:** High coverage expected (CI runs coverage on every PR).

**View Coverage:**
```bash
cargo tarpaulin --workspace --out Html
```

## Test Types

**Unit Tests:**
- In-module tests for specific logic/structs.
- `crates/seisly_core/src/types.rs`: `test_provenance_creation`.

**Integration Tests:**
- Tests covering multiple crates/components.
- `tests/phase1_integration_test.rs`: `test_ml_and_attributes_pipeline`.

**E2E Tests:**
- Not fully implemented, though integration tests cover high-level CLI and app logic.

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn test_something_async() {
    let result = some_async_fn().await;
    assert!(result.is_ok());
}
```

**Error Testing:**
```rust
#[test]
fn test_failure_case() {
    let result = problematic_fn();
    assert!(result.is_err());
}
```

---

*Testing analysis: 2026-03-31*
