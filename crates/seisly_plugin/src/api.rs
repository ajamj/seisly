//! Plugin API Definition

use serde_json::Value;
use thiserror::Error;

/// Plugin error types
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Initialization error: {0}")]
    InitError(String),
}

pub type Result<T> = std::result::Result<T, PluginError>;

/// Plugin context passed during initialization
pub struct PluginContext {
    pub version: String,
}

/// Plugin command metadata
pub struct PluginCommand {
    pub name: String,
    pub description: String,
}

/// Plugin trait - all plugins must implement this
pub trait Plugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Plugin description
    fn description(&self) -> &str;
    
    /// Initialize plugin
    fn initialize(&mut self, _ctx: PluginContext) -> Result<()> {
        Ok(())
    }
    
    /// Shutdown plugin
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// List available commands
    fn commands(&self) -> Vec<PluginCommand>;
    
    /// Execute a command
    fn execute(&self, cmd: &str, args: Value) -> Result<Value>;
}
