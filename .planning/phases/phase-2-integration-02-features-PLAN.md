---
phase: Phase 2 (v0.4.0)
plan: phase-2-integration-02-features
type: feature
autonomous: true
wave: 2
depends_on: ["phase-2-integration-01-foundation"]
files_modified: 5
requirements: ["v0.4.0-features"]
objective: Implement QI Analysis, 4D Monitoring metrics, and Async GPU Polling.
must_haves:
  truths:
    - "Quantitative Interpretation and 4D results are calculated accurately and displayed in the UI."
    - "GPU computations trigger an immediate UI update via ctx.request_repaint()."
  artifacts:
    - path: "crates/seisly_app/src/widgets/qi_panel.rs"
      provides: "Functional UI for AVO and Elastic Parameters"
      min_lines: 100
    - path: "crates/seisly_app/src/widgets/time_lapse_panel.rs"
      provides: "Functional UI for 4D survey monitoring"
      min_lines: 100
  key_links:
    - from: "crates/seisly_app/src/widgets/qi_panel.rs"
      to: "crates/seisly_app/src/app.rs"
      via: "Method signature expansion for GpuAttributeComputer access"
    - from: "crates/seisly_app/src/widgets/time_lapse_panel.rs"
      to: "crates/seisly_4d/src/lib.rs"
      via: "NRMS computation"
---

# Plan: Phase 2 Integration - 02 Features

## 1. Context and Objective
With the foundation in place, this plan implements the user-facing logic for Quantitative Interpretation and 4D monitoring, including the plumbing for asynchronous attribute computation.

## 2. Technical Strategy
- **Widget Communication:** Expand `QiPanel::ui` and `TimeLapsePanel::ui` to accept `&mut Option<GpuAttributeComputer>` and `&Sender<AppMessage>`.
- **QI Logic:** Map UI sliders and text inputs to `seisly_qi` rock physics formulas.
- **4D Logic:** Handle survey selection and NRMS calculation.
- **Async Polling:** Update `AppMessage` channel to handle GPU results and use `ctx.request_repaint()` to ensure results are processed immediately.

## 3. Work Units

<task id="WU-014">
  <name>QI Feature Implementation</name>
  <files>
    - crates/seisly_app/src/widgets/qi_panel.rs
  </files>
  <action>
    Implement AVO classification UI. Update `QiPanel::ui` to allow users to input angle/amplitude pairs. Use `seisly_qi::AvoAnalysis` to compute classes. Add a button for Vp/Vs ratio computation.
  </action>
  <verify>
    QI panel correctly calculates and displays AVO classes based on user input.
  </verify>
  <done>
    - QI tools are functional.
  </done>
</task>

<task id="WU-015">
  <name>4D Monitoring Implementation</name>
  <files>
    - crates/seisly_app/src/widgets/time_lapse_panel.rs
  </files>
  <action>
    Implement dropdowns for selecting Base and Monitor volumes from the project context. Add a button to trigger NRMS computation using `seisly_4d::TimeLapseMonitor`. Display the percentage result.
  </action>
  <verify>
    NRMS values are computed and visible in the panel.
  </verify>
  <done>
    - 4D monitoring functional.
  </done>
</task>

<task id="WU-016">
  <name>Async Attribute Polling & Integration</name>
  <files>
    - crates/seisly_app/src/app.rs
  </files>
  <action>
    Add `GpuAttributeResult(Vec<f32>)` to the `AppMessage` enum. In `SeislyApp::update`, handle this variant. **Crucial:** Ensure background threads call `ctx.request_repaint()` after sending messages via `tx`.
  </action>
  <verify>
    `cargo check` passes and UI remains interactive during computation.
  </verify>
  <done>
    - Async result bridge established with immediate redraw.
  </done>
</task>

<task id="WU-017">
  <name>Attribute UI Wiring</name>
  <files>
    - crates/seisly_app/src/app.rs
    - crates/seisly_app/src/widgets/qi_panel.rs
  </files>
  <action>
    Update `QiPanel::ui` and `SeislyApp::render_sidebar` signatures. Add "Compute RMS" button to the QI panel. When clicked, spawn a thread that calls `gpu_computer.compute_rms_gpu`, sends the result via `tx`, and calls `ctx.request_repaint()`.
  </action>
  <verify>
    Clicking computation buttons triggers logs and shows results immediately upon completion.
  </verify>
  <done>
    - Feature dispatch complete.
  </done>
</task>

## 4. Success Criteria
- [x] QI AVO Analysis functional.
- [x] 4D NRMS computation functional.
- [x] Async GPU computations do not block UI and results trigger immediate redraw.
