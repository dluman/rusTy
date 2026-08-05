use crate::{utils::with_gil, SpaCyError};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Typed wrapper around spaCy `MorphAnalysis`.
#[derive(Debug, Clone)]
pub struct MorphAnalysis {
    pub(crate) obj: Py<PyAny>,
}

impl MorphAnalysis {
    pub(crate) fn new(obj: Py<PyAny>) -> Self {
        MorphAnalysis { obj }
    }

    /// Return the analysis as a UD FEATS string (e.g. "Number=Sing|Gender=Masc").
    pub fn to_string(&self) -> Result<String, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let s = obj.str()?;
            let s: String = s.extract()?;
            Ok(s)
        })
    }

    /// Return the analysis as a `HashMap<String, String>`.
    pub fn to_dict(&self) -> Result<HashMap<String, String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let dict = obj.call_method0("to_dict")?;
            let items = dict.call_method0("items")?;
            let mut result = HashMap::new();
            for item in items.iter()? {
                let item = item?;
                let key: String = item.call_method1("__getitem__", (0,))?.extract()?;
                let value: String = item.call_method1("__getitem__", (1,))?.extract()?;
                result.insert(key, value);
            }
            Ok(result)
        })
    }

    /// Retrieve values for a feature by field.
    pub fn get(&self, field: &str) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let values = obj.call_method1("get", (field,))?;
            let mut result = Vec::new();
            for v in values.iter()? {
                result.push(v?.extract()?);
            }
            Ok(result)
        })
    }

    /// Check whether a feature/value pair is in the analysis.
    pub fn contains(&self, feature_value: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let contains: bool = obj
                .call_method1("__contains__", (feature_value,))?
                .extract()?;
            Ok(contains)
        })
    }

    /// Number of features in the analysis.
    pub fn len(&self) -> Result<usize, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let len: usize = obj.len()?;
            Ok(len)
        })
    }

    pub fn is_empty(&self) -> Result<bool, SpaCyError> {
        Ok(self.len()? == 0)
    }

    /// All feature/value pairs as strings (e.g. "Number=Sing").
    pub fn features(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let mut result = Vec::new();
            for feature in obj.iter()? {
                result.push(feature?.extract()?);
            }
            Ok(result)
        })
    }
}
