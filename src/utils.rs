use crate::error::SpaCyError;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::{prelude::*, types::PyDict};
use std::collections::HashMap;

/// Acquire the GIL and run a closure, converting PyErr to SpaCyError.
pub fn with_gil<T, F>(f: F) -> Result<T, SpaCyError>
where
    F: FnOnce(Python<'_>) -> Result<T, SpaCyError>,
{
    Python::with_gil(f)
}

/// Extract a Vec<f32> from a spaCy vector (numpy array) via rust-numpy.
pub fn extract_vec_f32(obj: &Bound<'_, PyAny>) -> Result<Vec<f32>, SpaCyError> {
    // spaCy vectors are numpy arrays; try to downcast to PyArray1<f32>
    let array = obj
        .downcast::<PyArray1<f32>>()
        .map_err(|e| SpaCyError::Numpy(format!("Failed to downcast to PyArray1<f32>: {}", e)))?;
    let vec = array
        .to_vec()
        .map_err(|e| SpaCyError::Numpy(format!("Failed to convert numpy array to vec: {}", e)))?;
    Ok(vec)
}

/// Build a PyDict from an optional JSON string for kwargs.
pub fn kwargs_from_json<'a>(
    py: Python<'a>,
    json: Option<&str>,
) -> Result<Option<Bound<'a, PyDict>>, SpaCyError> {
    match json {
        Some(s) => {
            let dict: Bound<'a, PyDict> = py
                .import_bound("json")?
                .getattr("loads")?
                .call1((s,))?
                .extract()?;
            Ok(Some(dict))
        }
        None => Ok(None),
    }
}

/// Convert a `serde_json::Value` into a Python object.
pub fn value_to_pyobject(py: Python, value: &serde_json::Value) -> Result<PyObject, SpaCyError> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.into_py(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_py(py))
            } else {
                Ok(n.as_f64().unwrap_or(0.0).into_py(py))
            }
        }
        serde_json::Value::String(s) => Ok(s.into_py(py)),
        serde_json::Value::Array(arr) => {
            let list = pyo3::types::PyList::empty_bound(py);
            for item in arr {
                list.append(value_to_pyobject(py, item)?)?;
            }
            Ok(list.into())
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new_bound(py);
            for (k, v) in obj {
                dict.set_item(k, value_to_pyobject(py, v)?)?;
            }
            Ok(dict.into())
        }
    }
}

/// Convert a slice of `serde_json::Value`s into a Python list.
pub fn values_to_pylist<'a>(
    py: Python<'a>,
    values: &[serde_json::Value],
) -> Result<Bound<'a, pyo3::types::PyList>, SpaCyError> {
    let list = pyo3::types::PyList::empty_bound(py);
    for value in values {
        list.append(value_to_pyobject(py, value)?)?;
    }
    Ok(list)
}

/// Convert a `HashMap<String, serde_json::Value>` into a Python dict.
pub fn values_to_pydict<'a>(
    py: Python<'a>,
    values: &'a HashMap<String, serde_json::Value>,
) -> Result<Bound<'a, PyDict>, SpaCyError> {
    let dict = PyDict::new_bound(py);
    for (k, v) in values {
        dict.set_item(k, value_to_pyobject(py, v)?)?;
    }
    Ok(dict)
}

/// Convert an arbitrary Python object into a `serde_json::Value` via `json.dumps`.
pub fn pyobject_to_json(obj: &Bound<'_, PyAny>) -> Result<serde_json::Value, SpaCyError> {
    let py = obj.py();
    let json_str: String = py
        .import_bound("json")?
        .getattr("dumps")?
        .call1((obj,))?
        .extract()?;
    let value = serde_json::from_str(&json_str)?;
    Ok(value)
}
