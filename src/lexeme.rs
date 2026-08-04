use crate::{utils::with_gil, SpaCyError};
use pyo3::prelude::*;

/// The lexical type of a token.
#[derive(Debug, Clone)]
pub struct Lexeme {
    pub(crate) obj: Py<PyAny>,
}

impl Lexeme {
    pub(crate) fn new(obj: Py<PyAny>) -> Self {
        Lexeme { obj }
    }

    pub fn orth_(&self) -> Result<String, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let val: String = obj.getattr("orth_")?.extract()?;
            Ok(val)
        })
    }

    pub fn rank(&self) -> Result<u64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let val: u64 = obj.getattr("rank")?.extract()?;
            Ok(val)
        })
    }

    pub fn prob(&self) -> Result<f64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let val: f64 = obj.getattr("prob")?.extract()?;
            Ok(val)
        })
    }

    pub fn cluster(&self) -> Result<u64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let val: u64 = obj.getattr("cluster")?.extract()?;
            Ok(val)
        })
    }

    pub fn has_vector(&self) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let val: bool = obj.getattr("has_vector")?.extract()?;
            Ok(val)
        })
    }

    pub fn vector(&self) -> Result<Vec<f32>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let vec = obj.getattr("vector")?;
            crate::utils::extract_vec_f32(&vec)
        })
    }
}
