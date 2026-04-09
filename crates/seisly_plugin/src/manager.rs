use crate::api::{Plugin, PluginCommand, PluginError, Result};
use crate::manifest::PluginManifest;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A temporary placeholder for a discovered but not yet fully loaded plugin
pub struct PlaceholderPlugin {
    manifest: PluginManifest,
}

impl PlaceholderPlugin {
    pub fn new(manifest: PluginManifest) -> Self {
        Self { manifest }
    }
}

impl Plugin for PlaceholderPlugin {
    fn name(&self) -> &str {
        &self.manifest.name
    }

    fn version(&self) -> &str {
        &self.manifest.version
    }

    fn description(&self) -> &str {
        self.manifest
            .description
            .as_deref()
            .unwrap_or("No description")
    }

    fn commands(&self) -> Vec<PluginCommand> {
        vec![]
    }

    fn execute(&self, _cmd: &str, _args: serde_json::Value) -> Result<serde_json::Value> {
        Err(PluginError::ExecutionError(
            "Plugin not yet fully loaded".to_string(),
        ))
    }
}

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
    pub fn execute(
        &self,
        name: &str,
        cmd: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let plugin = self
            .plugins
            .get(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;

        plugin.execute(cmd, args)
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    /// Discover plugins from directory
    pub fn discover(&mut self, path: &Path) -> Result<()> {
        if !path.exists() || !path.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(path).map_err(|e| PluginError::ExecutionError(e.to_string()))? {
            let entry = entry.map_err(|e| PluginError::ExecutionError(e.to_string()))?;
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("manifest.yaml");
                if manifest_path.exists() {
                    match PluginManifest::from_file(&manifest_path) {
                        Ok(manifest) => {
                            #[cfg(feature = "python")]
                            {
                                self.register(Box::new(crate::python_plugin::PythonPlugin::new(
                                    manifest,
                                    manifest_path,
                                )));
                            }
                            #[cfg(not(feature = "python"))]
                            {
                                self.register(Box::new(PlaceholderPlugin::new(manifest)));
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse manifest at {:?}: {}", manifest_path, e);
                        }
                    }
                }
            }
        }
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
