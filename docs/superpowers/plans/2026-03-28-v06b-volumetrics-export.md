# StrataForge v0.6b: Volumetrics & Export Management Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a grid-based volumetric engine and an export manager for geophysical and interoperability formats.

**Architecture:** A `VolumetricEngine` in `sf_compute` that evaluates surfaces on a common grid. An `ExportManager` in `sf_io` that serializes interpretation data. UI updates in `sf_app` to support multi-selection and reporting.

**Tech Stack:** Rust, ndarray, serde.

---

### Task 1: Grid-Based Volumetric Engine

**Files:**
- Create: `crates/sf_compute/src/volumetrics.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [ ] **Step 1: Implement grid evaluation logic**
Evaluate two surfaces (Upper/Lower) on a regular (x, y) grid.

- [ ] **Step 2: Implement volume integration**
Sum the $(Upper - Lower) \times dx \times dy$ for all cells where $Upper > Lower$.

- [ ] **Step 3: Add unit tests for constant thickness volume**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_compute/src/volumetrics.rs crates/sf_compute/src/lib.rs
git commit -m "feat: implement grid-based volumetric calculation engine"
```

---

### Task 2: Multi-Selection UI & Volumetric Reporting

**Files:**
- Modify: `crates/sf_app/src/app.rs`
- Modify: `crates/sf_app/src/interpretation/mod.rs`

- [ ] **Step 1: Update Interpretation Explorer to allow multi-selection**

- [ ] **Step 2: Add "Calculate Volume" dialog**
Trigger the volumetric engine and display results in a floating panel.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/
git commit -m "ui: implement multi-selection and volumetric reporting"
```

---

### Task 3: Geophysical & Interoperability Export

**Files:**
- Create: `crates/sf_io/src/export/mod.rs`
- Create: `crates/sf_io/src/export/json.rs`
- Create: `crates/sf_io/src/export/xyz.rs`
- Modify: `crates/sf_io/src/lib.rs`

- [ ] **Step 1: Implement XYZ export for surfaces**

- [ ] **Step 2: Implement StrataForge JSON schema and export**
Serialize picks, faults, and metadata to a single 3D JSON file.

- [ ] **Step 3: Add "Export" button to the Analysis toolbar**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_io/src/export/ crates/sf_io/src/lib.rs crates/sf_app/src/app.rs
git commit -m "feat: implement XYZ and JSON data export"
```
