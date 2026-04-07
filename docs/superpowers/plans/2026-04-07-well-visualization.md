# Well Data & Visualization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `.LAS` file importing and interactive well log plotting (Gamma Ray, Density, etc.) in the UI.

**Architecture:** Use `seisly_io::las::parser::LasParser` to load the well data into `seisly_core::domain::well::Well`. Store the well in `InterpretationState::wells`. Update `WellPanel` to render the active well's logs using `egui_plot`.

**Tech Stack:** Rust, egui, egui_plot, seisly_io, seisly_core.

---

### Task 1: Add UI Dependencies

**Files:**
- Modify: `crates/seisly_app/Cargo.toml`

- [ ] **Step 1: Add egui_plot dependency**
```toml
egui_plot = "0.29.0"
```

- [ ] **Step 2: Commit**
```bash
git add crates/seisly_app/Cargo.toml
git commit -m "build: add egui_plot dependency for well visualization"
```

---

### Task 2: Implement LAS Import Logic

**Files:**
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Implement import_well logic**
In `crates/seisly_app/src/app.rs`, locate `import_well` and use `rfd::FileDialog` to pick a file. Call `seisly_io::las::parser::LasParser::read(path)` and add the resulting `Well` to `self.wells.wells`. Also set it as the active well.

```rust
    fn import_well(&mut self) {
        use rfd::FileDialog;
        if let Some(path) = FileDialog::new()
            .set_title("Import Well Data")
            .add_filter("LAS File", &["las"])
            .pick_file()
        {
            match seisly_io::las::parser::LasParser::read(&path) {
                Ok(mut well) => {
                    // Use file stem as name if empty
                    if well.name.is_empty() {
                        well.name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                    }
                    self.wells.active_well_id = Some(well.id);
                    self.wells.wells.push(well);
                    log::info!("Successfully imported well: {:?}", path);
                }
                Err(e) => {
                    log::error!("Failed to import well: {}", e);
                }
            }
        }
    }
```

- [ ] **Step 2: Check compilation**
Run `cargo check -p seisly_app` to ensure there are no borrow checker or typing issues.

- [ ] **Step 3: Commit**
```bash
git add crates/seisly_app/src/app.rs
git commit -m "feat: implement LAS file import logic"
```

---

### Task 3: Implement Well Log Plotting

**Files:**
- Modify: `crates/seisly_app/src/widgets/well_panel.rs`
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Setup WellPanel struct**
Update `WellPanel` to track which log curve is selected for the active well. Add a field `selected_curve_id: Option<uuid::Uuid>`.

- [ ] **Step 2: Render well selection and curve list**
In `crates/seisly_app/src/app.rs` inside `render_well_logs`, loop through `self.wells.wells` to allow selecting an active well. Then list its available logs (`well.logs`) as selectable buttons.

- [ ] **Step 3: Render the Plot**
Using `egui_plot::Plot`, render the selected log curve. The x-axis should be the log value and the y-axis should be depth.
*Note: In well logging, depth increases downwards, so the y-axis might need to be inverted by convention, but a standard plot is fine for V1.*

- [ ] **Step 4: Check compilation**
Run `cargo check -p seisly_app`.

- [ ] **Step 5: Commit**
```bash
git add crates/seisly_app/src/widgets/well_panel.rs crates/seisly_app/src/app.rs
git commit -m "feat: implement well log plotting using egui_plot"
```
