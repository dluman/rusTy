use pyo3::{prelude::*, types::PyDict};
use numpy::PyArray1;
use crate::error::SpaCyError;

/// Acquire the GIL and run a closure, converting PyErr to SpaCyError.
pub fn with_gil<T, F>(f: F) -> Result<T, SpaCyError>
where
    F: FnOnce(Python<'_>) -> Result<T, SpaCyError>,
{
    Python::with_gil(|py| f(py))
}

/// Extract a Vec<f32> from a spaCy vector (numpy array) via rust-numpy.
pub fn extract_vec_f32(obj: &Bound<'_, PyAny>) -> Result<Vec<f32>, SpaCyError> {
    // spaCy vectors are numpy arrays; try to downcast to PyArray1<f32>
    let array = obj.downcast::<PyArray1<f32>>()
        .map_err(|e| SpaCyError::Numpy(format!("Failed to downcast to PyArray1<f32>: {}", e)))?;
    let vec = array.to_vec()?;
    Ok(vec)
}

/// Build a PyDict from an optional JSON string for kwargs.
pub fn kwargs_from_json<'a>(
    py: Python<'a>,
    json: Option<&str>,
) -> Result<Option<&'a PyDict>, SpaCyError> {
    match json {
        Some(s) => {
            let dict: &PyDict = py.import("json")?
                .getattr("loads")?
                .call1((s,))?
                .extract()?;
            Ok(Some(dict))
        }
        None => Ok(None),
    }
}
