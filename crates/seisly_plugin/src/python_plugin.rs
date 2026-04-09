use crate::api::{Plugin, PluginCommand, PluginError, Result};
use crate::ipc::IpcBridge;
use crate::manifest::PluginManifest;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;

/// A plugin implemented in Python, now running in a separate process.
pub struct PythonPlugin {
    manifest: PluginManifest,
    path: PathBuf,
    ipc: Arc<IpcBridge>,
}

impl PythonPlugin {
    pub fn new(manifest: PluginManifest, path: PathBuf) -> Self {
        Self {
            manifest,
            path,
            ipc: Arc::new(IpcBridge::new()),
        }
    }
}

impl Plugin for PythonPlugin {
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
        vec![PluginCommand {
            name: "run".to_string(),
            description: "Run the Python plugin (isolated)".to_string(),
        }]
    }

    fn execute(&self, _cmd: &str, args: Value) -> Result<Value> {
        let plugin_dir = self
            .path
            .parent()
            .ok_or_else(|| PluginError::ExecutionError("Invalid plugin path".to_string()))?;

        let module_name = self.manifest.entry_point.trim_end_matches(".py");

        let params = json!({
            "plugin_dir": plugin_dir.to_str().unwrap(),
            "module_name": module_name,
            "args": args
        });

        self.ipc
            .execute("execute", params)
            .map_err(|e| PluginError::ExecutionError(e.to_string()))
    }
}
