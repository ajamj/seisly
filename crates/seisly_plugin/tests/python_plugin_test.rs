use seisly_plugin::PluginManager;
use std::fs;
use tempfile::tempdir;

#[test]
#[cfg(feature = "python")]
fn test_python_plugin_execution() {
    pyo3::prepare_freethreaded_python();

    let dir = tempdir().unwrap();
    let plugin_dir = dir.path().join("test_python_plugin");
    fs::create_dir(&plugin_dir).unwrap();

    let manifest_yaml = r#"
name: TestPythonPlugin
version: 0.1.0
plugin_type: horizon
entry_point: main.py
"#;
    fs::write(plugin_dir.join("manifest.yaml"), manifest_yaml).unwrap();

    let python_code = r#"
def execute(args):
    print("Python plugin executing...")
    return {"status": "success"}
"#;
    fs::write(plugin_dir.join("main.py"), python_code).unwrap();

    let mut manager = PluginManager::new();
    manager.discover(dir.path()).unwrap();

    assert_eq!(manager.plugin_count(), 1);
    let result = manager
        .execute("TestPythonPlugin", "run", serde_json::Value::Null)
        .unwrap();
    assert_eq!(
        result,
        serde_json::Value::String("Python execution successful".to_string())
    );
}
