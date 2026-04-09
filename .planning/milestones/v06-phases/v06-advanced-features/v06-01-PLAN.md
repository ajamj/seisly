---
phase: v06-advanced-features
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - Cargo.toml
  - crates/seisly_plugin/Cargo.toml
  - crates/seisly_plugin/src/lib.rs
  - crates/seisly_plugin/src/interpreter.rs
  - crates/seisly_plugin/src/manifest.rs
  - crates/seisly_plugin/src/manager.rs
autonomous: true
requirements: [R7.1]
must_haves:
  truths:
    - "Python interpreter initializes without crashing"
    - "PluginManager finds mock plugins in plugins/ directory"
    - "Manifests are correctly parsed into PluginManifest structs"
  artifacts:
    - path: "crates/seisly_plugin/src/interpreter.rs"
      provides: "Python initialization logic"
    - path: "crates/seisly_plugin/src/manifest.rs"
      provides: "YAML manifest parsing"
  key_links:
    - from: "crates/seisly_plugin/src/manager.rs"
      to: "crates/seisly_plugin/src/manifest.rs"
      via: "parse_manifest function"
---

<objective>
Initialize the embedded Python interpreter using PyO3 and implement the plugin discovery mechanism that scans for YAML manifests.

Purpose: Provides the foundation for running Python-based AI models directly inside the Rust application.
Output: Working Python bridge and plugin discovery system.
</objective>

<execution_context>
@$HOME/.gemini/get-shit-done/workflows/execute-plan.md
@$HOME/.gemini/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md
@docs/superpowers/specs/2026-03-30-plugin-python-ai-design.md
@docs/superpowers/plans/2026-03-30-plugin-python-ai-implementation.md

# Existing plugin manager
@crates/seisly_plugin/src/lib.rs
@crates/seisly_plugin/src/manager.rs
</context>

<tasks>

<task type="auto" tdd="true">
  <name>Task 1: Workspace Setup & Python Interpreter Initialization</name>
  <files>Cargo.toml, crates/seisly_plugin/Cargo.toml, crates/seisly_plugin/src/interpreter.rs, crates/seisly_plugin/src/lib.rs</files>
  <behavior>
    - Test 1: Calling PythonInterpreter::new() succeeds and allows running "import sys; sys.version".
    - Test 2: Verify that the "python" feature in seisly_plugin correctly enables PyO3.
  </behavior>
  <action>
    - Add pyo3 (v0.20) and numpy (v0.20) to workspace dependencies in root Cargo.toml.
    - Update crates/seisly_plugin/Cargo.toml to include pyo3 and serde_yaml.
    - Implement PythonInterpreter in crates/seisly_plugin/src/interpreter.rs with new() and run_string() methods.
    - Use pyo3::prepare_freethreaded_python() for initialization.
    - Export the module in crates/seisly_plugin/src/lib.rs.
  </action>
  <verify>
    <automated>cargo test -p seisly_plugin --features python</automated>
  </verify>
  <done>Python interpreter can be initialized and execute strings in a test environment.</done>
</task>

<task type="auto" tdd="true">
  <name>Task 2: Plugin Manifest Parsing & Discovery</name>
  <files>crates/seisly_plugin/src/manifest.rs, crates/seisly_plugin/src/manager.rs</files>
  <behavior>
    - Test 1: PluginManifest::from_yaml can parse a standard manifest.yaml string.
    - Test 2: PluginManager::discover(path) finds subdirectories with manifest.yaml and populates the plugin list.
  </behavior>
  <action>
    - Define PluginManifest struct in manifest.rs (name, version, type, entry_point).
    - Implement YAML deserialization using serde_yaml.
    - Update PluginManager::discover in manager.rs to scan the provided path recursively (or one level deep) for directories containing "manifest.yaml".
    - For each found manifest, create a placeholder plugin entry (actual PythonPlugin implementation comes in Plan 02).
  </action>
  <verify>
    <automated>cargo test -p seisly_plugin --features python</automated>
  </verify>
  <done>Plugin discovery logic correctly identifies and parses manifest files from a directory.</done>
</task>

</tasks>

<verification>
- Run `cargo test -p seisly_plugin --features python` to verify both interpreter init and discovery logic.
- Verify that `plugins/` directory is created (can be done in Task 2 or as a post-step).
</verification>

<success_criteria>
- Python 3.x interpreter is successfully embedded.
- Mock manifests in a temporary directory are discovered and parsed correctly.
</success_criteria>

<output>
After completion, create `.planning/phases/v06-advanced-features/v06-01-SUMMARY.md`
</output>
