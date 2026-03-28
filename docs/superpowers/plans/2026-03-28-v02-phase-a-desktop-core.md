# StrataForge v0.2 Phase A: Desktop Core & 3D Viewer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Establish the foundational desktop application using `egui` and `eframe`, with a dedicated `wgpu` viewport for 3D visualization.

**Architecture:** A three-panel layout (Data Explorer, 3D Viewport, Analysis Panel) built with `eframe`. The 3D Viewport uses `egui::PaintCallback` to integrate native `wgpu` rendering.

**Tech Stack:** Rust, eframe (egui), wgpu.

---

### Task 0: Application Dependency Setup

**Files:**
- Modify: `crates/sf_app/Cargo.toml`

- [x] **Step 1: Add necessary eframe features**
- [x] **Step 2: Commit**

---

### Task 1: Scaffolding the eframe Application

**Files:**
- Create: `crates/sf_app/src/app.rs`
- Modify: `crates/sf_app/src/main.rs`

- [x] **Step 1: Write the basic App structure in app.rs**
- [x] **Step 2: Update main.rs to launch the app**
- [x] **Step 3: Run the app to verify it launches**
- [x] **Step 4: Commit**

---

### Task 2: Implementing the Three-Panel Layout (Project Explorer)

**Files:**
- Modify: `crates/sf_app/src/app.rs`

- [x] **Step 1: Update the update() method with SidePanels and Collapsing headers**
- [x] **Step 2: Run and verify layout**
- [x] **Step 3: Commit**

---

### Task 3: Integrating wgpu Viewport (PaintCallback Verification)

**Files:**
- Create: `crates/sf_app/src/widgets/viewport.rs`
- Create: `crates/sf_app/src/widgets/mod.rs`
- Modify: `crates/sf_app/src/app.rs`

- [x] **Step 1: Create a ViewportWidget with visual confirmation in PaintCallback**
- [x] **Step 2: Add mod.rs in widgets folder**
- [x] **Step 3: Update StrataForgeApp state in app.rs**
- [x] **Step 4: Update update() method to use the widget**
- [x] **Step 5: Run and verify**
- [x] **Step 6: Commit**
