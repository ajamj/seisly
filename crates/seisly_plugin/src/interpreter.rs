//! Embedded Python interpreter for running Python-based plugins.

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Embedded Python interpreter.
#[cfg(feature = "python")]
pub struct PythonInterpreter;

#[cfg(feature = "python")]
impl PythonInterpreter {
    /// Initialize a new Python interpreter.
    pub fn new() -> PyResult<Self> {
        pyo3::prepare_freethreaded_python();
        Ok(Self)
    }

    /// Execute a Python string.
    pub fn run_string(&self, code: &str) -> PyResult<()> {
        Python::with_gil(|py| py.run_bound(code, None, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "python")]
    fn test_interpreter_init() {
        let interp = PythonInterpreter::new().expect("Failed to initialize Python");
        interp
            .run_string("import sys; print(f'Python version: {sys.version}')")
            .expect("Failed to run Python code");
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_python_feature_enabled() {
        assert!(cfg!(feature = "python"));
    }
}
