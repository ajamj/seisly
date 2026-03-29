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
    fn execute(&self, py: Python, plugin_name: &str, command: &str, args: &PyDict) -> PyResult<PyObject> {
        // Convert PyDict to serde_json::Value
        let args_json = python_to_json(py, args)?;
        
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
fn python_to_json(py: Python, dict: &PyDict) -> PyResult<serde_json::Value> {
    let json_str = dict.to_string();
    serde_json::from_str(&json_str)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

/// Helper: Convert JSON to Python object
fn json_to_python(py: Python, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::String(s) => Ok(s.into_py(py)),
        serde_json::Value::Number(n) => Ok(n.as_f64().unwrap_or(0.0).into_py(py)),
        serde_json::Value::Bool(b) => Ok(b.into_py(py)),
        serde_json::Value::Null => Ok(py.None()),
        _ => Ok("{}".into_py(py)),
    }
}

/// Module definition
#[pymodule]
pub fn stratforge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPluginManager>()?;
    Ok(())
}
