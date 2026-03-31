use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use numpy::PyArrayDyn;
use ndarray::ArrayViewD;

pub mod shm;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    id: u64,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    id: u64,
    result: Option<serde_json::Value>,
    error: Option<String>,
}

fn main() -> anyhow::Result<()> {
    // Initialize Python
    pyo3::prepare_freethreaded_python();
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    // Use a loop to process requests from stdin
    while handle.read_line(&mut line)? > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        let req: Request = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = Response {
                    id: 0,
                    result: None,
                    error: Some(format!("Parse error: {}", e)),
                };
                if let Ok(s) = serde_json::to_string(&err_resp) {
                    println!("{}", s);
                }
                line.clear();
                continue;
            }
        };

        let response = handle_request(req);
        if let Ok(s) = serde_json::to_string(&response) {
            println!("{}", s);
            io::stdout().flush()?;
        }
        
        line.clear();
    }

    Ok(())
}

fn handle_request(req: Request) -> Response {
    let result = Python::with_gil(|py| -> Result<serde_json::Value, String> {
        match req.method.as_str() {
            "execute" => {
                // Expected params: { "plugin_dir": "...", "module_name": "...", "args": {} }
                let plugin_dir = req.params["plugin_dir"].as_str()
                    .ok_or("Missing plugin_dir")?;
                let module_name = req.params["module_name"].as_str()
                    .ok_or("Missing module_name")?;
                let _args = &req.params["args"];

                let sys = py.import_bound("sys").map_err(|e| e.to_string())?;
                let path = sys.getattr("path").map_err(|e| e.to_string())?;
                
                // Add plugin directory to sys.path
                path.call_method1("append", (plugin_dir,)).map_err(|e| e.to_string())?;

                // Import the module
                let module = py.import_bound(module_name).map_err(|e| e.to_string())?;
                let execute_fn = module.getattr("execute").map_err(|e| e.to_string())?;
                
                // Prepare arguments
                let py_args = PyDict::new_bound(py);
                
                // Call execute
                let py_result = execute_fn.call1((py_args,)).map_err(|e| e.to_string())?;
                
                // Convert result to JSON
                python_to_json(py_result).map_err(|e| e.to_string())
            }
            "ping" => {
                Ok(serde_json::Value::String("pong".to_string()))
            }
            "load_shm" => {
                // Params: { "shm_id": "...", "shape": [...], "dtype": "..." }
                let shm_id = req.params["shm_id"].as_str().ok_or("Missing shm_id")?;
                let shape: Vec<usize> = req.params["shape"].as_array().ok_or("Missing shape")?
                    .iter().map(|v| v.as_u64().unwrap_or(0) as usize).collect();
                let dtype = req.params["dtype"].as_str().unwrap_or("f32");

                let array = shm::map_shm_to_numpy(py, shm_id, shape, dtype).map_err(|e| e.to_string())?;
                
                // For verification, return some stats
                let sum = array.call_method0("sum").map_err(|e| e.to_string())?;
                python_to_json(sum).map_err(|e| e.to_string())
            }
            _ => Err(format!("Unknown method: {}", req.method)),
        }
    });

    match result {
        Ok(val) => Response {
            id: req.id,
            result: Some(val),
            error: None,
        },
        Err(e) => Response {
            id: req.id,
            result: None,
            error: Some(e),
        },
    }
}

fn python_to_json(obj: Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        return Ok(serde_json::Value::Null);
    }
    if let Ok(s) = obj.extract::<String>() {
        return Ok(serde_json::Value::String(s));
    }
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(serde_json::Value::Bool(b));
    }
    if let Ok(n) = obj.extract::<f64>() {
        return Ok(serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap()));
    }
    // For complex types, just use a placeholder for now or convert to string
    Ok(serde_json::Value::String(obj.str()?.to_string()))
}

/// NumPy bridging logic moved from seisly_plugin
pub fn share_with_python<'py>(
    py: Python<'py>,
    data: &[f32],
    shape: Vec<usize>,
) -> PyResult<Bound<'py, PyArrayDyn<f32>>> {
    let view = ArrayViewD::from_shape(shape, data)
        .map_err(|e: ndarray::ShapeError| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    
    // Safety: The caller must ensure the data slice outlives the NumPy array
    let array = unsafe {
        PyArrayDyn::borrow_from_array_bound(&view, py.None().into_bound(py))
    };
    Ok(array)
}
