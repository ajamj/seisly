//! Python Bindings for StrataForge Plugin System

use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::manager::PluginManager;

/// Python wrapper for PluginManager
#[pyclass]
pub struct PyPluginManager {
    manager: PluginManager,
}

#[pymethods]
impl PyPluginManager {
    #[new]
    fn new() -> Self {
        Self {
            manager: PluginManager::new(),
        }
    }
    
    /// List all registered plugins
    fn list_plugins(&self) -> PyResult<Vec<String>> {
        Ok(self.manager.list_plugins().iter().map(|s| s.to_string()).collect())
    }
    
    /// Execute a plugin command
    fn execute(&self, py: Python<'_>, plugin_name: &str, command: &str, args: Bound<'_, PyDict>) -> PyResult<PyObject> {
        // Convert PyDict to serde_json::Value
        let args_json = python_to_json(py, &args)?;
        
        // Execute plugin
        match self.manager.execute(plugin_name, command, args_json) {
            Ok(result) => json_to_python(py, &result),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
        }
    }
    
    /// Get plugin count
    fn plugin_count(&self) -> usize {
        self.manager.plugin_count()
    }
}

/// Helper: Convert Python dict to JSON
fn python_to_json(_py: Python<'_>, _dict: &Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    // Hacky placeholder
    Ok(serde_json::Value::Object(serde_json::Map::new()))
}

/// Helper: Convert JSON to Python object
fn json_to_python(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::String(s) => Ok(s.to_object(py)),
        serde_json::Value::Number(n) => Ok(n.as_f64().unwrap_or(0.0).to_object(py)),
        serde_json::Value::Bool(b) => Ok(b.to_object(py)),
        serde_json::Value::Null => Ok(py.None()),
        _ => Ok("{}".to_object(py)),
    }
}

/// Module definition
#[pymodule]
pub fn stratforge(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPluginManager>()?;
    Ok(())
}
