# StrataForge v0.2 Phase A: Desktop Core & 3D Viewer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish the foundational desktop application using `egui` and `eframe`, with a dedicated `wgpu` viewport for 3D visualization.

**Architecture:** A three-panel layout (Data Explorer, 3D Viewport, Analysis Panel) built with `eframe`. The 3D Viewport uses `egui::PaintCallback` to integrate native `wgpu` rendering.

**Tech Stack:** Rust, eframe (egui), wgpu.

---

### Task 0: Application Dependency Setup

**Files:**
- Modify: `crates/sf_app/Cargo.toml`

- [ ] **Step 1: Add necessary eframe features**

```toml
[dependencies]
# ... existing workspace dependencies ...
eframe = { workspace = true, features = ["wgpu"] }
# ensure wgpu, egui, tokio are already in workspace.dependencies
```

- [ ] **Step 2: Commit**

```bash
git add crates/sf_app/Cargo.toml
git commit -m "chore: configure sf_app dependencies for wgpu"
```

---

### Task 1: Scaffolding the eframe Application

**Files:**
- Create: `crates/sf_app/src/app.rs`
- Modify: `crates/sf_app/src/main.rs`

- [ ] **Step 1: Write the basic App structure in app.rs**

```rust
use eframe::egui;

pub struct StrataForgeApp {
    name: String,
}

impl StrataForgeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "MyField".to_owned(),
        }
    }
}

impl eframe::App for StrataForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("StrataForge");
            ui.label(format!("Project: {}", self.name));
        });
    }
}
```

- [ ] **Step 2: Update main.rs to launch the app**

```rust
mod app;
use app::StrataForgeApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "StrataForge",
        native_options,
        Box::new(|cc| Box::new(StrataForgeApp::new(cc))),
    )
}
```

- [ ] **Step 3: Run the app to verify it launches**

Run: `cargo run --bin sf-app`
Expected: A window opens showing "StrataForge" and "Project: MyField".

- [ ] **Step 4: Commit**

```bash
git add crates/sf_app/src/app.rs crates/sf_app/src/main.rs
git commit -m "feat: scaffold eframe desktop application"
```

---

### Task 2: Implementing the Three-Panel Layout (Project Explorer)

**Files:**
- Modify: `crates/sf_app/src/app.rs`

- [ ] **Step 1: Update the update() method with SidePanels and Collapsing headers**

```rust
impl eframe::App for StrataForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Project Data");
            ui.separator();
            
            ui.collapsing("Seismic Volumes", |ui| {
                ui.label("None loaded");
            });
            
            ui.collapsing("Wells", |ui| {
                ui.label("Well-1");
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("AI Analysis");
            ui.separator();
            ui.button("Run Fault Detection");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("3D Viewport");
            ui.label("Viewport goes here");
        });
    }
}
```

- [ ] **Step 2: Run and verify layout**

Run: `cargo run --bin sf-app`
Expected: Left panel with collapsible headers, Right panel, and Central panel are visible.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/app.rs
git commit -m "ui: implement project explorer and side panels"
```

---

### Task 3: Integrating wgpu Viewport (PaintCallback Verification)

**Files:**
- Create: `crates/sf_app/src/widgets/viewport.rs`
- Create: `crates/sf_app/src/widgets/mod.rs`
- Modify: `crates/sf_app/src/app.rs`

- [ ] **Step 1: Create a ViewportWidget with visual confirmation in PaintCallback**

```rust
use eframe::egui;
use eframe::wgpu;

pub struct ViewportWidget {}

impl ViewportWidget {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let (rect, _response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());
        
        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(eframe::egui_wgpu::CallbackFn::new(
                |_info, _render_pass, _renderer| {
                    // No-op for now, but verifies pass integration
                    // In a real pass, we would clear or draw here.
                },
            )),
        };
        ui.painter().add(callback);
        
        // Add a visual fallback to confirm the widget area is correctly allocated
        ui.painter().rect_stroke(rect, 0.0, (1.0, egui::Color32::DARK_GRAY));
    }
}
```

- [ ] **Step 2: Add mod.rs in widgets folder**

```rust
pub mod viewport;
```

- [ ] **Step 3: Update StrataForgeApp state in app.rs**

```rust
mod widgets;
use widgets::viewport::ViewportWidget;

pub struct StrataForgeApp {
    name: String,
    viewport: ViewportWidget,
}

impl StrataForgeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "MyField".to_owned(),
            viewport: ViewportWidget::new(),
        }
    }
}
```

- [ ] **Step 4: Update update() method to use the widget**

```rust
        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui);
        });
```

- [ ] **Step 5: Run and verify**

Run: `cargo run --bin sf-app`
Expected: Application launches, central area shows a thin gray border (allocated viewport area).

- [ ] **Step 6: Commit**

```bash
git add crates/sf_app/src/widgets/ crates/sf_app/src/app.rs
git commit -m "feat: add viewport widget with wgpu PaintCallback verification"
```
