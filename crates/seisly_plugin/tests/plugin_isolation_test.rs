use seisly_plugin::PluginManager;
use std::fs;
use tempfile::tempdir;

#[test]
#[cfg(feature = "python")]
fn test_python_plugin_isolation() {
    let dir = tempdir().unwrap();
    let plugin_dir = dir.path().join("crash_plugin");
    fs::create_dir(&plugin_dir).unwrap();

    let manifest = r#"
name: crash_plugin
version: 0.1.0
plugin_type: fault
entry_point: plugin.py
"#;
    fs::write(plugin_dir.join("manifest.yaml"), manifest).unwrap();

    let plugin_py = r#"
import os

def execute(params):
    # Hard crash the worker process
    os._exit(1)
"#;
    fs::write(plugin_dir.join("plugin.py"), plugin_py).unwrap();

    let mut manager = PluginManager::new();
    manager.discover(dir.path()).unwrap();

    assert_eq!(manager.plugin_count(), 1);

    // First execution should fail because of crash
    let result = manager.execute("crash_plugin", "run", serde_json::json!({}));
    assert!(result.is_err(), "Execution should have failed due to crash");

    // The main process (the test) should still be running!

    // Second execution with a FIXED plugin should work (verifies restart logic)
    let plugin_py_ok = r#"
def execute(params):
    return "recovered"
"#;
    fs::write(plugin_dir.join("plugin.py"), plugin_py_ok).unwrap();

    // We might need a small delay or retry if the OS hasn't fully cleaned up the process
    // but IpcBridge::ensure_worker should handle it via try_wait.

    let result2 = manager.execute("crash_plugin", "run", serde_json::json!({}));
    assert!(
        result2.is_ok(),
        "Second execution should succeed after restart: {:?}",
        result2.err()
    );
    assert_eq!(result2.unwrap(), "recovered");
}
