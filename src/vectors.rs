use crate::utils::{extract_vec_f32, with_gil};
use crate::{SpaCyError, Vocab};
use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// spaCy's vector storage.
#[derive(Debug, Clone)]
pub struct Vectors {
    pub(crate) obj: Py<PyAny>,
}

impl Vectors {
    pub(crate) fn new(obj: Py<PyAny>) -> Self {
        Vectors { obj }
    }

    /// Create a new empty vector table.
    /// `shape` is `(rows, dims)`.
    pub fn new_table(vocab: &Vocab, shape: Option<(usize, usize)>) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.vectors")?.getattr("Vectors")?;
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("strings", vocab.obj.bind(py).getattr("strings")?)?;
            if let Some((rows, dims)) = shape {
                kwargs.set_item("shape", (rows, dims))?;
            }
            let vectors = cls.call((), Some(&kwargs))?;
            Ok(Vectors {
                obj: vectors.into(),
            })
        })
    }

    /// Add a key to the table, optionally setting its vector or mapping it to an existing row.
    pub fn add(
        &self,
        key: &str,
        vector: Option<&[f32]>,
        row: Option<usize>,
    ) -> Result<usize, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            if let Some(vec) = vector {
                let arr = PyArray1::from_vec_bound(py, vec.to_vec());
                kwargs.set_item("vector", arr)?;
            }
            if let Some(r) = row {
                kwargs.set_item("row", r)?;
            }
            let result: usize = obj.call_method("add", (key,), Some(&kwargs))?.extract()?;
            Ok(result)
        })
    }

    /// Get the vector for a key.
    pub fn get(&self, key: &str) -> Result<Vec<f32>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let vec = obj.call_method1("__getitem__", (key,))?;
            extract_vec_f32(&vec)
        })
    }

    /// Check whether a key is in the table.
    pub fn contains(&self, key: u64) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let contains: bool = obj.call_method1("__contains__", (key,))?.extract()?;
            Ok(contains)
        })
    }

    /// Number of rows in the table.
    pub fn len(&self) -> Result<usize, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let len: usize = obj.len()?;
            Ok(len)
        })
    }

    pub fn is_empty(&self) -> Result<bool, SpaCyError> {
        Ok(self.keys()?.is_empty())
    }

    /// All keys in the table (as integer hashes).
    pub fn keys(&self) -> Result<Vec<u64>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let keys = obj.call_method0("keys")?;
            let mut result = Vec::new();
            for key in keys.iter()? {
                result.push(key?.extract()?);
            }
            Ok(result)
        })
    }

    /// Look up a key's row or a row's key.
    /// Returns the row index (if key given) or the key hash (if row given).
    pub fn find(&self, key: Option<&str>, row: Option<usize>) -> Result<i64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            if let Some(k) = key {
                kwargs.set_item("key", k)?;
            }
            if let Some(r) = row {
                kwargs.set_item("row", r)?;
            }
            let result: i64 = obj.call_method("find", (), Some(&kwargs))?.extract()?;
            Ok(result)
        })
    }

    /// Find the `n` most similar vectors for each query.
    /// Returns `(keys, best_rows, scores)` where each inner Vec corresponds to one query.
    #[allow(clippy::type_complexity)]
    pub fn most_similar(
        &self,
        queries: &[Vec<f32>],
        n: usize,
        batch_size: usize,
    ) -> Result<(Vec<Vec<u64>>, Vec<Vec<u64>>, Vec<Vec<f64>>), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);

            // Build 2D numpy array from queries
            let n_queries = queries.len();
            let dims = queries.first().map(|v| v.len()).unwrap_or(0);
            let flat: Vec<f32> = queries.iter().flat_map(|v| v.iter().copied()).collect();
            let arr2 = numpy::ndarray::Array2::from_shape_vec((n_queries, dims), flat)
                .map_err(|e| SpaCyError::Numpy(e.to_string()))?;
            let py_queries = arr2.into_pyarray_bound(py);

            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("n", n)?;
            kwargs.set_item("batch_size", batch_size)?;
            let result = obj.call_method("most_similar", (py_queries,), Some(&kwargs))?;

            // Helper to extract a 2D list from a numpy array via tolist()
            let extract_2d_u64 = |arr: &Bound<'_, PyAny>| -> Result<Vec<Vec<u64>>, SpaCyError> {
                let list = arr.call_method0("tolist")?;
                let mut outer = Vec::new();
                for row in list.iter()? {
                    let row = row?;
                    let mut inner = Vec::new();
                    for val in row.iter()? {
                        inner.push(val?.extract()?);
                    }
                    outer.push(inner);
                }
                Ok(outer)
            };

            let extract_2d_f64 = |arr: &Bound<'_, PyAny>| -> Result<Vec<Vec<f64>>, SpaCyError> {
                let list = arr.call_method0("tolist")?;
                let mut outer = Vec::new();
                for row in list.iter()? {
                    let row = row?;
                    let mut inner = Vec::new();
                    for val in row.iter()? {
                        inner.push(val?.extract()?);
                    }
                    outer.push(inner);
                }
                Ok(outer)
            };

            let keys = extract_2d_u64(&result.get_item(0)?)?;
            let rows = extract_2d_u64(&result.get_item(1)?)?;
            let scores = extract_2d_f64(&result.get_item(2)?)?;

            Ok((keys, rows, scores))
        })
    }

    /// Table shape as `(rows, dims)`.
    pub fn shape(&self) -> Result<(usize, usize), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let shape = obj.getattr("shape")?;
            let row: usize = shape.get_item(0)?.extract()?;
            let col: usize = shape.get_item(1)?.extract()?;
            Ok((row, col))
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let bytes: Vec<u8> = obj.call_method0("to_bytes")?.extract()?;
            Ok(bytes)
        })
    }

    pub fn from_bytes(vocab: &Vocab, bytes: &[u8]) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.vectors")?.getattr("Vectors")?;
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("strings", vocab.obj.bind(py).getattr("strings")?)?;
            let vectors = cls.call((), Some(&kwargs))?;
            let vectors = vectors.call_method1("from_bytes", (bytes,))?;
            Ok(Vectors {
                obj: vectors.into(),
            })
        })
    }

    pub fn to_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("to_disk", (path,))?;
            Ok(())
        })
    }

    pub fn from_disk(vocab: &Vocab, path: &str) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.vectors")?.getattr("Vectors")?;
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("strings", vocab.obj.bind(py).getattr("strings")?)?;
            let vectors = cls.call((), Some(&kwargs))?;
            let vectors = vectors.call_method1("from_disk", (path,))?;
            Ok(Vectors {
                obj: vectors.into(),
            })
        })
    }
}
