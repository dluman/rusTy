use pyo3::prelude::*;
use crate::{SpaCyError, utils::with_gil};

#[derive(Debug, Clone)]
pub struct Vocab {
    pub(crate) obj: Py<PyAny>,
}

impl Vocab {
    pub(crate) fn new(obj: Py<PyAny>) -> Self {
        Vocab { obj }
    }

    pub fn strings(&self) -> Result<StringStore, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let strings = obj.getattr("strings")?;
            Ok(StringStore { obj: strings.into() })
        })
    }
}

#[derive(Debug, Clone)]
pub struct StringStore {
    pub(crate) obj: Py<PyAny>,
}

impl StringStore {
    pub fn add(&self, string: &str) -> Result<u64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let hash = obj.call_method1("add", (string,))?;
            let hash: u64 = hash.extract()?;
            Ok(hash)
        })
    }

    pub fn get_hash(&self, string: &str) -> Result<u64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let hash = obj.call_method1("__getitem__", (string,))?;
            let hash: u64 = hash.extract()?;
            Ok(hash)
        })
    }

    pub fn get_string(&self, hash: u64) -> Result<String, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let string = obj.call_method1("__getitem__", (hash,))?;
            let string: String = string.extract()?;
            Ok(string)
        })
    }
}
