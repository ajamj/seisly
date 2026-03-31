use numpy::{Element, PyArrayDyn};
use pyo3::prelude::*;
use seisly_core::ipc::ShmSegment;
use ndarray::ArrayViewMutD;

/// Maps a Shared Memory segment into a NumPy array.
pub fn map_shm_to_numpy<'py>(
    py: Python<'py>,
    shm_id: &str,
    shape: Vec<usize>,
    dtype: &str,
) -> PyResult<Bound<'py, PyAny>> {
    // Open the shared memory segment
    let seg = ShmSegment::open(shm_id)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to open SHM: {}", e)))?;

    // Check size
    let element_size = match dtype {
        "f32" => std::mem::size_of::<f32>(),
        "f64" => std::mem::size_of::<f64>(),
        "i32" => std::mem::size_of::<i32>(),
        "u8" => std::mem::size_of::<u8>(),
        _ => return Err(pyo3::exceptions::PyValueError::new_err(format!("Unsupported dtype: {}", dtype))),
    };

    let num_elements: usize = shape.iter().product();
    let total_size = num_elements * element_size;

    if total_size > seg.size() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "SHM segment size ({}) is smaller than requested array size ({})",
            seg.size(),
            total_size
        )));
    }

    let array = match dtype {
        "f32" => unsafe { create_array_from_ptr::<f32>(py, seg.as_ptr() as *mut f32, shape.clone())?.into_any() },
        "f64" => unsafe { create_array_from_ptr::<f64>(py, seg.as_ptr() as *mut f64, shape.clone())?.into_any() },
        "i32" => unsafe { create_array_from_ptr::<i32>(py, seg.as_ptr() as *mut i32, shape.clone())?.into_any() },
        "u8" => unsafe { create_array_from_ptr::<u8>(py, seg.as_ptr() as *mut u8, shape.clone())?.into_any() },
        _ => unreachable!(),
    };

    // To keep the segment alive, we can attach it to the array's base or just leak it for now
    std::mem::forget(seg);

    Ok(array)
}

unsafe fn create_array_from_ptr<'py, T: Element>(
    py: Python<'py>,
    ptr: *mut T,
    shape: Vec<usize>,
) -> PyResult<Bound<'py, PyArrayDyn<T>>> {
    let view = ArrayViewMutD::from_shape_ptr(shape, ptr);
    let array = PyArrayDyn::borrow_from_array_bound(&view, py.None().into_bound(py));
    Ok(array)
}
