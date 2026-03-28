# StrataForge v0.2 Phase B: Seismic Data Slicing & I/O Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement SEG-Y data loading and efficient volume slicing for 2D visualization and AI analysis.

**Architecture:** Extend `sf_io` with a `SegyParser` and `sf_compute` with a `VolumeSlicer`. Slicing will use memory-mapped access for high performance.

**Tech Stack:** Rust, `segy-rs` (if available) or custom parser, `memmap2`.

---

### Task 1: Scaffolding SEG-Y Parser

**Files:**
- Create: `crates/sf_io/src/segy/mod.rs`
- Create: `crates/sf_io/src/segy/parser.rs`
- Modify: `crates/sf_io/src/lib.rs`

- [ ] **Step 1: Define basic SEG-Y metadata types in parser.rs**

```rust
pub struct SegyMetadata {
    pub inline_range: (i32, i32),
    pub crossline_range: (i32, i32),
    pub sample_count: usize,
    pub sample_interval: f32,
}
```

- [ ] **Step 2: Implement a skeleton parse_metadata function**

```rust
pub fn parse_metadata(path: &std::path::Path) -> anyhow::Result<SegyMetadata> {
    // Placeholder: In reality, read binary/text headers
    Ok(SegyMetadata {
        inline_range: (1, 100),
        crossline_range: (1, 100),
        sample_count: 500,
        sample_interval: 4.0,
    })
}
```

- [ ] **Step 3: Register segy module in lib.rs**

```rust
pub mod segy;
pub use segy::parser::LasParser; // and others
```

- [ ] **Step 4: Commit**

```bash
git add crates/sf_io/src/segy/ crates/sf_io/src/lib.rs
git commit -m "feat: scaffold SEG-Y parser"
```

---

### Task 2: Implementing Volume Slicer in sf_compute

**Files:**
- Create: `crates/sf_compute/src/seismic.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [ ] **Step 1: Define SeismicVolume and Slicer**

```rust
pub struct SeismicVolume {
    pub data: Vec<f32>, // Flat array for now
    pub width: usize,   // Inlines
    pub height: usize,  // Xlines
    pub depth: usize,   // Samples
}

impl SeismicVolume {
    pub fn get_inline(&self, inline_idx: usize) -> Vec<f32> {
        // Extract a 2D slice from the 3D volume
        let mut slice = Vec::with_capacity(self.height * self.depth);
        // ... extraction logic ...
        slice
    }
}
```

- [ ] **Step 2: Add unit tests for slicing**

- [ ] **Step 3: Commit**

```bash
git add crates/sf_compute/src/seismic.rs crates/sf_compute/src/lib.rs
git commit -m "feat: add seismic volume slicing logic"
```
