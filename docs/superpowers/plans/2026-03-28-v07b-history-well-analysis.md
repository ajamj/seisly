# StrataForge v0.7b: Interactive History & Well Analysis Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a robust interpretation history stack (Undo/Redo) and advanced 3D well-log visualization with cross-plotting capabilities.

**Architecture:** Use the Command Pattern for interpretation actions. Implement an `HistoryManager` in `sf_app`. Extend `sf_render` with a `LogRenderer` for 3D strips. Add a `CrossPlotWidget` using `egui_plot`.

**Tech Stack:** Rust, egui_plot.

---

### Task 1: Undo/Redo Command Stack

**Files:**
- Create: `crates/sf_app/src/interpretation/history.rs`
- Modify: `crates/sf_app/src/interpretation/mod.rs`

- [ ] **Step 1: Define InterpretationCommand trait**
Actions: `AddPick`, `DeletePick`, `AutoTrack`, `GenerateSurface`.

- [ ] **Step 2: Implement HistoryManager**
Manage `undo_stack` and `redo_stack` of commands.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/interpretation/history.rs
git commit -m "feat: implement undo/redo command stack for interpretation"
```

---

### Task 2: 3D Well-Log Strips

**Files:**
- Create: `crates/sf_render/src/logs.rs`
- Modify: `crates/sf_render/src/lib.rs`

- [ ] **Step 1: Implement LogRenderer**
Generate 3D strip or tube geometry from well log data and trajectory.

- [ ] **Step 2: Add Log shading**
Update shaders to color logs based on amplitude (e.g., Gamma Ray value).

- [ ] **Step 3: Commit**

```bash
git add crates/sf_render/src/logs.rs
git commit -m "feat: implement 3D well-log strip rendering"
```

---

### Task 3: Cross-plot Analysis Widget

**Files:**
- Create: `crates/sf_app/src/widgets/crossplot.rs`
- Modify: `crates/sf_app/src/app.rs`

- [ ] **Step 1: Implement CrossPlotWidget using egui_plot**
Scatter plot showing two log attributes (or attribute vs depth).

- [ ] **Step 2: Implement 3D-to-Plot selection bridge**
Clicking a 3D log point highlights it in the cross-plot.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/widgets/crossplot.rs
git commit -m "ui: add interactive cross-plot analysis widget"
```
