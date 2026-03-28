# StrataForge v0.1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build offline-first Rust workspace with project format, SQLite storage, CRS support, LAS/trajectory/XYZ import, and basic 3D rendering.

**Architecture:** Monorepo Cargo workspace with 8 crates (sf_core, sf_crs, sf_storage, sf_io, sf_compute, sf_render, sf_cli, sf_app). LocalBackend implements Backend trait for offline mode.

**Tech Stack:** Rust, egui, wgpu, winit, rusqlite, proj (PROJ bindings), blake3, rayon, spade (triangulation), serde, tracing.

---

## File Structure

### Crates to Create
```
strataforge/
  Cargo.toml                          # workspace definition
  crates/
    sf_core/                          # domain model (pure Rust, no native deps)
    sf_crs/                           # PROJ wrappers
    sf_storage/                       # SQLite + blob store
    sf_io/                            # Import/export parsers
    sf_compute/                       # Algorithms
    sf_render/                        # wgpu primitives
    sf_cli/                           # CLI commands
    sf_app/                           # egui desktop app (stub for v0.1)
  schemas/
    sqlite/
      0001_init.sql
  docs/
    architecture.md
    project_format.md
    roadmap.md
  examples/
    sample_wells/
  .github/workflows/
    ci.yml
```

---

## Task Breakdown

### Task 1: Workspace Skeleton

**Files:**
- Create: `Cargo.toml`, `crates/sf_core/Cargo.toml`, `crates/sf_crs/Cargo.toml`, `crates/sf_storage/Cargo.toml`, `crates/sf_io/Cargo.toml`, `crates/sf_compute/Cargo.toml`, `crates/sf_render/Cargo.toml`, `crates/sf_cli/Cargo.toml`, `crates/sf_app/Cargo.toml`
- Create: `.github/workflows/ci.yml`, `rust-toolchain.toml`, `.gitignore`

- [ ] **Step 1: Create workspace Cargo.toml**
- [ ] **Step 2: Create individual crate Cargo.toml files**
- [ ] **Step 3: Create rust-toolchain.toml**
- [ ] **Step 4: Create .gitignore**
- [ ] **Step 5: Create CI workflow**
- [ ] **Step 6: Commit**

### Task 2: Core Domain Model (sf_core)

**Files:**
- Create: `crates/sf_core/src/lib.rs`, `crates/sf_core/src/types.rs`, `crates/sf_core/src/crs.rs`, `crates/sf_core/src/domain/mod.rs`, `crates/sf_core/src/domain/well.rs`, `crates/sf_core/src/domain/trajectory.rs`, `crates/sf_core/src/domain/log.rs`, `crates/sf_core/src/domain/surface.rs`
- Test: `crates/sf_core/tests/domain_tests.rs`

- [ ] **Step 1: Write Crs type definition**
- [ ] **Step 2: Write Provenance type**
- [ ] **Step 3: Write Well domain type**
- [ ] **Step 4: Write Trajectory domain type**
- [ ] **Step 5: Write Log domain type**
- [ ] **Step 6: Write Surface domain type**
- [ ] **Step 7: Write lib.rs exports**
- [ ] **Step 8: Write tests**
- [ ] **Step 9: Run tests**
- [ ] **Step 10: Commit**

### Task 3: CRS Transformer (sf_crs)

**Files:**
- Create: `crates/sf_crs/src/lib.rs`, `crates/sf_crs/src/transformer.rs`, `crates/sf_crs/src/registry.rs`
- Test: `crates/sf_crs/tests/transform_tests.rs`

- [ ] **Step 1: Write Transformer implementation**
- [ ] **Step 2: Write CRS registry**
- [ ] **Step 3: Write lib.rs**
- [ ] **Step 4: Write transform tests**
- [ ] **Step 5: Run tests**
- [ ] **Step 6: Commit**

### Task 4: Storage Layer (sf_storage)

**Files:**
- Create: `crates/sf_storage/src/lib.rs`, `crates/sf_storage/src/project.rs`, `crates/sf_storage/src/sqlite/mod.rs`, `crates/sf_storage/src/sqlite/schema.rs`, `crates/sf_storage/src/sqlite/connection.rs`, `crates/sf_storage/src/blob/mod.rs`, `crates/sf_storage/src/blob/store.rs`
- Create: `schemas/sqlite/0001_init.sql`
- Test: `crates/sf_storage/tests/storage_tests.rs`

- [ ] **Step 1: Write SQLite schema**
- [ ] **Step 2: Write project.yaml parser**
- [ ] **Step 3: Write SQLite connection wrapper**
- [ ] **Step 4: Write blob store**
- [ ] **Step 5: Write lib.rs**
- [ ] **Step 6: Write integration tests**
- [ ] **Step 7: Add tempfile dev dependency**
- [ ] **Step 8: Run tests**
- [ ] **Step 9: Commit**

### Task 5: IO Parsers (sf_io)

**Files:**
- Create: `crates/sf_io/src/lib.rs`, `crates/sf_io/src/las/mod.rs`, `crates/sf_io/src/las/parser.rs`, `crates/sf_io/src/csv/mod.rs`, `crates/sf_io/src/csv/trajectory.rs`, `crates/sf_io/src/xyz/mod.rs`, `crates/sf_io/src/xyz/surface.rs`
- Test: `crates/sf_io/tests/io_tests.rs`
- Example data: `examples/sample_wells/well1.las`, `examples/sample_wells/well1_traj.csv`, `examples/sample_wells/surface1.xyz`

- [ ] **Step 1: Create sample LAS file**
- [ ] **Step 2: Create sample trajectory CSV**
- [ ] **Step 3: Create sample XYZ surface**
- [ ] **Step 4: Write LAS parser**
- [ ] **Step 5: Write trajectory CSV parser**
- [ ] **Step 6: Write XYZ surface parser**
- [ ] **Step 7: Write lib.rs**
- [ ] **Step 8: Write IO tests**
- [ ] **Step 9: Run tests**
- [ ] **Step 10: Commit**

### Task 6: Compute Layer (sf_compute)

**Files:**
- Create: `crates/sf_compute/src/lib.rs`, `crates/sf_compute/src/triangulation.rs`, `crates/sf_compute/src/resampling.rs`
- Test: `crates/sf_compute/tests/compute_tests.rs`

- [ ] **Step 1: Write triangulation using spade**
- [ ] **Step 2: Write trajectory resampling**
- [ ] **Step 3: Write lib.rs**
- [ ] **Step 4: Write compute tests**
- [ ] **Step 5: Run tests**
- [ ] **Step 6: Commit**

### Task 7: CLI Commands (sf_cli)

**Files:**
- Create: `crates/sf_cli/src/main.rs`, `crates/sf_cli/src/commands/mod.rs`, `crates/sf_cli/src/commands/init.rs`, `crates/sf_cli/src/commands/import.rs`, `crates/sf_cli/src/commands/list.rs`
- Test: Manual testing

- [ ] **Step 1: Write CLI main with clap**
- [ ] **Step 2: Write init command**
- [ ] **Step 3: Write import command**
- [ ] **Step 4: Write list command**
- [ ] **Step 5: Write commands/mod.rs**
- [ ] **Step 6: Update sf_cli Cargo.toml**
- [ ] **Step 7: Build and test CLI**
- [ ] **Step 8: Manual testing**
- [ ] **Step 9: Commit**

### Task 8: Documentation

**Files:**
- Create: `docs/architecture.md`, `docs/project_format.md`, `docs/roadmap.md`, `README.md`

- [ ] **Step 1: Write README.md**
- [ ] **Step 2: Write architecture.md**
- [ ] **Step 3: Write project_format.md**
- [ ] **Step 4: Write roadmap.md**
- [ ] **Step 5: Commit**

---

## Timeline

| Phase | Duration |
|-------|----------|
| Task 1: Workspace Skeleton | 30 min |
| Task 2: Core Domain Model | 60 min |
| Task 3: CRS Transformer | 45 min |
| Task 4: Storage Layer | 60 min |
| Task 5: IO Parsers | 60 min |
| Task 6: Compute Layer | 45 min |
| Task 7: CLI Commands | 60 min |
| Task 8: Documentation | 30 min |
| **Total** | **~6.5 hours** |

---

## Rollback Plan

If issues occur during implementation:

1. **Identify failing task** - Check which test/command fails
2. **Revert last commit** - `git reset --hard HEAD~1`
3. **Fix incrementally** - Re-implement the failing task with smaller steps
4. **Run full test suite** - `cargo test --workspace`
5. **Verify CI passes** - Push to branch and check GitHub Actions

---

## Security Checklist

- [x] Input validation (LAS/CSV parsers handle malformed input)
- [ ] Auth checks (N/A for offline mode, required for server mode)
- [ ] Rate limiting (N/A for CLI)
- [x] Error handling (thiserror + anyhow throughout)
- [ ] Path traversal protection (validate project paths)
- [ ] Blob hash verification (verify BLAKE3 on retrieve)

---

## Plan Review Loop

**Next step:** Dispatch plan-document-reviewer subagent to review this implementation plan.
