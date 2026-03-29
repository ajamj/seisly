//! Plugin System Tests

use sf_plugin::{PluginManager, Plugin, PluginContext, PluginCommand, Result};
use serde_json::Value;

struct TestPlugin;

impl Plugin for TestPlugin {
    fn name(&self) -> &str { "TestPlugin" }
    fn version(&self) -> &str { "1.0.0" }
    fn description(&self) -> &str { "Test plugin for unit testing" }
    
    fn commands(&self) -> Vec<PluginCommand> {
        vec![PluginCommand {
            name: "test".to_string(),
            description: "Test command".to_string(),
        }]
    }
    
    fn execute(&self, cmd: &str, _args: Value) -> Result<Value> {
        if cmd == "test" {
            Ok(Value::String("success".to_string()))
        } else {
            Err(sf_plugin::api::PluginError::ExecutionError("Unknown command".to_string()))
        }
    }
}

#[test]
fn test_plugin_registration() {
    let mut manager = PluginManager::new();
    manager.register(Box::new(TestPlugin));
    
    let plugins = manager.list_plugins();
    assert!(plugins.contains(&"TestPlugin"));
    assert_eq!(manager.plugin_count(), 1);
}

#[test]
fn test_plugin_execution() {
    let mut manager = PluginManager::new();
    manager.register(Box::new(TestPlugin));
    
    let result = manager.execute("TestPlugin", "test", Value::Null).unwrap();
    assert_eq!(result, Value::String("success".to_string()));
}

#[test]
fn test_plugin_not_found() {
    let manager = PluginManager::new();
    let result = manager.execute("NonExistent", "test", Value::Null);
    assert!(result.is_err());
    match result {
        Err(sf_plugin::api::PluginError::NotFound(_)) => (),
        _ => panic!("Expected NotFound error"),
    }
}

#[test]
fn test_multiple_plugins() {
    let mut manager = PluginManager::new();
    
    struct PluginA;
    impl Plugin for PluginA {
        fn name(&self) -> &str { "PluginA" }
        fn version(&self) -> &str { "1.0" }
        fn description(&self) -> &str { "A" }
        fn commands(&self) -> Vec<PluginCommand> { vec![] }
        fn execute(&self, _: &str, _: Value) -> Result<Value> { Ok(Value::Null) }
    }
    
    struct PluginB;
    impl Plugin for PluginB {
        fn name(&self) -> &str { "PluginB" }
        fn version(&self) -> &str { "2.0" }
        fn description(&self) -> &str { "B" }
        fn commands(&self) -> Vec<PluginCommand> { vec![] }
        fn execute(&self, _: &str, _: Value) -> Result<Value> { Ok(Value::Null) }
    }
    
    manager.register(Box::new(PluginA));
    manager.register(Box::new(PluginB));
    
    assert_eq!(manager.plugin_count(), 2);
    let plugins = manager.list_plugins();
    assert!(plugins.contains(&"PluginA"));
    assert!(plugins.contains(&"PluginB"));
}
