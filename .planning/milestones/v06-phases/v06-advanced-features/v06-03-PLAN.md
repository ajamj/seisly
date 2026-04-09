---
phase: v06-advanced-features
plan: 03
type: execute
wave: 3
depends_on: ["v06-02"]
files_modified:
  - crates/seisly_app/src/widgets/mod.rs
  - crates/seisly_app/src/widgets/plugin_panel.rs
  - crates/seisly_app/src/app.rs
  - plugins/mock_horizon/manifest.yaml
  - plugins/mock_horizon/main.py
autonomous: false
requirements: [R7.1]
must_haves:
  truths:
    - "Plugin Manager window lists discovered Python plugins"
    - "Clicking 'Run' on a mock plugin generates horizon picks in the interpretation"
  artifacts:
    - path: "crates/seisly_app/src/widgets/plugin_panel.rs"
      provides: "egui UI for plugin management"
    - path: "plugins/mock_horizon/main.py"
      provides: "E2E verification script"
  key_links:
    - from: "crates/seisly_app/src/app.rs"
      to: "crates/seisly_app/src/widgets/plugin_panel.rs"
      via: "rendering logic"
---

<objective>
Implement the egui Plugin Manager UI and verify the entire Python AI architecture with a mock horizon tracking plugin.

Purpose: Provides user-facing controls for the new plugin system and confirms data flows correctly from Rust to Python and back.
Output: Plugin Manager UI and verified E2E workflow.
</objective>

<execution_context>
@$HOME/.gemini/get-shit-done/workflows/execute-plan.md
@$HOME/.gemini/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md
@.planning/phases/v06-advanced-features/v06-02-SUMMARY.md
@crates/seisly_app/src/app.rs
@crates/seisly_plugin/src/manager.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: egui Plugin Manager UI</name>
  <files>crates/seisly_app/src/widgets/plugin_panel.rs, crates/seisly_app/src/widgets/mod.rs, crates/seisly_app/src/app.rs</files>
  <action>
    - Create PluginPanel widget in plugin_panel.rs using egui.
    - Display a table/list of discovered plugins from PluginManager.
    - Implement a "Run" button for each plugin that triggers the execute() method.
    - Integrated the panel into SeislyApp in app.rs (add toggle and render call).
  </action>
  <verify>
    <automated>cargo check -p seisly_app</automated>
  </verify>
  <done>Plugin Manager UI is integrated and displays discovered plugins.</done>
</task>

<task type="auto">
  <name>Task 2: Mock Plugin & E2E Verification</name>
  <files>plugins/mock_horizon/manifest.yaml, plugins/mock_horizon/main.py</files>
  <action>
    - Create plugins/mock_horizon directory.
    - Write a manifest.yaml defining a "Mock Horizon Tracker".
    - Write a main.py that:
      - Imports numpy.
      - Receives a seismic volume slice (via zero-copy bridge).
      - Returns a hardcoded list of picks (e.g., a flat horizon at Z=500).
    - Implement the glue code in app.rs to handle results from the plugin and add them to the InterpretationState.
  </action>
  <verify>
    <automated>cargo test -p seisly_app --features python</automated>
  </verify>
  <done>Full E2E flow from UI to Python back to Rust is functional.</done>
</task>

<task type="checkpoint:human-verify" gate="blocking">
  <what-built>Plugin Manager UI and E2E workflow with Mock Plugin</what-built>
  <how-to-verify>
    - Launch the app: `cargo run --features python`
    - Open the Plugin Manager (find the new menu or button).
    - Verify "Mock Horizon Tracker" is listed.
    - Click "Run".
    - Verify that a new horizon appears in the viewport/interpretation list.
  </how-to-verify>
  <resume-signal>approved</resume-signal>
</task>

</tasks>

<verification>
- Launch app with `cargo run --features python` and verify visual results.
- Run `cargo test --workspace --features python` for final health check.
</verification>

<success_criteria>
- Plugin Manager window is usable and responsive.
- Python plugin execution results in persistent data changes in the interpretation.
</success_criteria>

<output>
After completion, create `.planning/phases/v06-advanced-features/v06-03-SUMMARY.md`
</output>
