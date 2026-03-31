use seisly_plugin::PluginManager;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_plugin_discovery() {
    let dir = tempdir().unwrap();
    let plugin_dir = dir.path().join("mock_plugin");
    fs::create_dir(&plugin_dir).unwrap();
    
    let manifest_yaml = r#"
name: MockPlugin
version: 0.1.0
plugin_type: horizon
entry_point: main.py
"#;
    fs::write(plugin_dir.join("manifest.yaml"), manifest_yaml).unwrap();
    
    let mut manager = PluginManager::new();
    manager.discover(dir.path()).unwrap();
    
    assert_eq!(manager.plugin_count(), 1);
    let plugins = manager.list_plugins();
    assert!(plugins.contains(&"MockPlugin"));
    
    let plugin = manager.execute("MockPlugin", "any", serde_json::Value::Null);
    assert!(plugin.is_err()); // Placeholder should return ExecutionError
}

#[test]
fn test_discovery_empty_dir() {
    let dir = tempdir().unwrap();
    let mut manager = PluginManager::new();
    manager.discover(dir.path()).unwrap();
    assert_eq!(manager.plugin_count(), 0);
}

#[test]
fn test_discovery_invalid_manifest() {
    let dir = tempdir().unwrap();
    let plugin_dir = dir.path().join("invalid_plugin");
    fs::create_dir(&plugin_dir).unwrap();
    fs::write(plugin_dir.join("manifest.yaml"), "invalid: yaml: :").unwrap();
    
    let mut manager = PluginManager::new();
    manager.discover(dir.path()).unwrap();
    assert_eq!(manager.plugin_count(), 0);
}
