//! Plugin Manager

use crate::api::{Plugin, PluginContext, PluginError, Result};
use std::collections::HashMap;
use std::path::Path;

/// Manages plugin registration and execution
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
    
    /// Register a plugin
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
    }
    
    /// Execute a plugin command
    pub fn execute(&self, name: &str, cmd: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        let plugin = self.plugins.get(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;
        
        plugin.execute(cmd, args)
    }
    
    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
    
    /// Discover plugins from directory
    pub fn discover(&mut self, _path: &Path) -> Result<()> {
        // TODO: Implement plugin discovery from directory
        Ok(())
    }
    
    /// Get plugin count
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
