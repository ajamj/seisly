---
phase: v06-advanced-features
plan: 02
type: execute
wave: 2
depends_on: ["v06-01"]
files_modified:
  - crates/seisly_plugin/src/lib.rs
  - crates/seisly_plugin/src/bridge.rs
  - crates/seisly_plugin/src/python_plugin.rs
  - crates/seisly_plugin/src/manager.rs
autonomous: true
requirements: [R7.1]
must_haves:
  truths:
    - "Rust slice & [f32] is shared with Python as a NumPy array without copying"
    - "Python scripts can be executed via the Plugin trait interface"
  artifacts:
    - path: "crates/seisly_plugin/src/bridge.rs"
      provides: "NumPy bridge logic"
    - path: "crates/seisly_plugin/src/python_plugin.rs"
      provides: "Plugin trait implementation for Python entry points"
  key_links:
    - from: "crates/seisly_plugin/src/python_plugin.rs"
      to: "crates/seisly_plugin/src/bridge.rs"
      via: "share_with_python call"
---

<objective>
Implement the high-performance data bridge between Rust and Python and provide a PythonPlugin struct that implements the standard Plugin trait.

Purpose: Enables zero-copy data sharing for massive seismic volumes and unifies Python scripts with the Rust plugin manager.
Output: Zero-copy bridge and working Python plugin execution.
</objective>

<execution_context>
@$HOME/.gemini/get-shit-done/workflows/execute-plan.md
@$HOME/.gemini/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md
@.planning/phases/v06-advanced-features/v06-01-SUMMARY.md
@crates/seisly_plugin/src/lib.rs
@crates/seisly_plugin/src/manager.rs
@crates/seisly_plugin/src/api.rs
</context>

<tasks>

<task type="auto" tdd="true">
  <name>Task 1: Zero-Copy Shared Memory Bridge</name>
  <files>crates/seisly_plugin/src/bridge.rs, crates/seisly_plugin/src/lib.rs</files>
  <behavior>
    - Test 1: Verify that share_with_python returns a numpy array that points to the same memory as the input slice.
    - Test 2: Modify the array in Python and assert that the Rust slice reflects the change (if using &mut [f32]).
  </behavior>
  <action>
    - Implement share_with_python function in bridge.rs using pyo3-numpy's borrow_from_array.
    - Support common data shapes (1D, 2D, 3D) for seismic volumes.
    - Export the bridge module in lib.rs.
  </action>
  <verify>
    <automated>cargo test -p seisly_plugin --features python</automated>
  </verify>
  <done>Rust-to-NumPy bridge provides zero-copy access to seismic data slices.</done>
</task>

<task type="auto" tdd="true">
  <name>Task 2: PythonPlugin Implementation</name>
  <files>crates/seisly_plugin/src/python_plugin.rs, crates/seisly_plugin/src/manager.rs, crates/seisly_plugin/src/lib.rs</files>
  <behavior>
    - Test 1: Creating a PythonPlugin pointing to a mock main.py correctly calls its "execute" function.
    - Test 2: PluginManager discovery (from Plan 01) now creates real PythonPlugin instances.
  </behavior>
  <action>
    - Create PythonPlugin struct implementing the Plugin trait (name, version, initialize, execute, etc.).
    - Use the PythonInterpreter from Plan 01 to load and call the Python entry points.
    - Update PluginManager::discover in manager.rs to instantiate PythonPlugin for discovered manifests.
  </action>
  <verify>
    <automated>cargo test -p seisly_plugin --features python</automated>
  </verify>
  <done>Python scripts are integrated as first-class plugins in the PluginManager.</done>
</task>

</tasks>

<verification>
- Run `cargo test -p seisly_plugin --features python` to verify zero-copy and plugin trait execution.
</verification>

<success_criteria>
- Zero-copy data sharing is verified by pointer comparison or shared mutation.
- A dummy Python script can be called through the standard Plugin interface.
</success_criteria>

<output>
After completion, create `.planning/phases/v06-advanced-features/v06-02-SUMMARY.md`
</output>
