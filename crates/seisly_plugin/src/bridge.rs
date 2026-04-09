use ndarray::ArrayViewD;
use numpy::{PyArrayDyn, PyArrayMethods};
use pyo3::prelude::*;

/// Size threshold for SHM transfer (1 MB in bytes)
const SHM_SIZE_THRESHOLD: usize = 1024 * 1024;

/// High-performance zero-copy bridge to Python.
#[cfg(feature = "python")]
pub fn share_with_python<'py>(
    py: Python<'py>,
    data: &[f32],
    shape: Vec<usize>,
) -> PyResult<Bound<'py, PyArrayDyn<f32>>> {
    let view = ArrayViewD::from_shape(shape, data)
        .map_err(|e: ndarray::ShapeError| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    // Safety: The caller must ensure the data slice outlives the NumPy array
    let array = unsafe { PyArrayDyn::borrow_from_array_bound(&view, py.None().into_bound(py)) };
    Ok(array)
}

/// Returns the size in bytes of the data slice
pub fn data_size_bytes(data: &[f32]) -> usize {
    data.len() * std::mem::size_of::<f32>()
}

/// Determines if data should be transferred via SHM based on size threshold
pub fn should_use_shm(data: &[f32]) -> bool {
    data_size_bytes(data) >= SHM_SIZE_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "python")]
    fn test_zero_copy_bridge() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = vec![1.0, 2.0, 3.0, 4.0];
            let shape = vec![4];
            let array = share_with_python(py, &data, shape).unwrap();

            // Check that the array has the correct values
            let readonly = array.readonly();
            let view = readonly.as_array();
            assert_eq!(view[0], 1.0);
            assert_eq!(view[3], 4.0);

            // Verify that the memory address is the same (zero-copy)
            let ptr = data.as_ptr() as usize;
            let data_ptr = array.data() as usize;
            assert_eq!(ptr, data_ptr);
        });
    }

    #[test]
    fn test_shm_threshold_detection() {
        // Small data (< 1MB) should not use SHM
        let small_data = vec![0.0f32; 1000]; // ~4KB
        assert!(!should_use_shm(&small_data));

        // Large data (>= 1MB) should use SHM
        let large_data = vec![0.0f32; 262144]; // 262144 * 4 = 1,048,576 bytes = 1MB
        assert!(should_use_shm(&large_data));

        // Just under threshold
        let just_under = vec![0.0f32; 262143]; // 1,048,572 bytes
        assert!(!should_use_shm(&just_under));
    }

    #[test]
    fn test_data_size_calculation() {
        let data = vec![0.0f32; 100];
        assert_eq!(data_size_bytes(&data), 400); // 100 * 4 bytes
    }
}
