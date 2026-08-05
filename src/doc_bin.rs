use crate::{utils::with_gil, Doc, SpaCyError, Vocab};
use pyo3::prelude::*;
use pyo3::types::PyList;

/// A binary serializer for collections of `Doc` objects.
/// More efficient than pickling and allows deserialization without
/// executing arbitrary Python code.
#[derive(Debug, Clone)]
pub struct DocBin {
    pub(crate) obj: Py<PyAny>,
}

impl DocBin {
    /// Create a new `DocBin`.
    ///
    /// `attrs` controls which token attributes are serialized.
    /// If `None`, spaCy's default set is used.
    /// `store_user_data` includes `Doc.user_data` and custom extensions.
    pub fn new(attrs: Option<&[&str]>, store_user_data: bool) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.tokens")?.getattr("DocBin")?;
            let kwargs = pyo3::types::PyDict::new_bound(py);
            if let Some(a) = attrs {
                let py_attrs = PyList::new_bound(py, a);
                kwargs.set_item("attrs", py_attrs)?;
            }
            kwargs.set_item("store_user_data", store_user_data)?;
            let bin = cls.call((), Some(&kwargs))?;
            Ok(DocBin { obj: bin.into() })
        })
    }

    /// Convenience constructor that creates a `DocBin` and immediately
    /// adds the given `Doc`s.
    pub fn from_docs(
        attrs: Option<&[&str]>,
        store_user_data: bool,
        docs: &[Doc],
    ) -> Result<Self, SpaCyError> {
        let bin = Self::new(attrs, store_user_data)?;
        for doc in docs {
            bin.add(doc)?;
        }
        Ok(bin)
    }

    /// Add a `Doc`'s annotations to the bin.
    pub fn add(&self, doc: &Doc) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = doc.obj.bind(py);
            obj.call_method1("add", (doc,))?;
            Ok(())
        })
    }

    /// Number of `Doc`s added to the bin.
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

    /// Merge another `DocBin` into this one.
    pub fn merge(&self, other: &DocBin) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let other = other.obj.bind(py);
            obj.call_method1("merge", (other,))?;
            Ok(())
        })
    }

    /// Serialize the bin to a `Vec<u8>`.
    pub fn to_bytes(&self) -> Result<Vec<u8>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let bytes: Vec<u8> = obj.call_method0("to_bytes")?.extract()?;
            Ok(bytes)
        })
    }

    /// Deserialize a `DocBin` from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.tokens")?.getattr("DocBin")?;
            let bin = cls.call((), None)?;
            let bin = bin.call_method1("from_bytes", (bytes,))?;
            Ok(DocBin { obj: bin.into() })
        })
    }

    /// Save the bin to disk (typically `.spacy` extension).
    pub fn to_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("to_disk", (path,))?;
            Ok(())
        })
    }

    /// Load a `DocBin` from disk.
    pub fn from_disk(path: &str) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.tokens")?.getattr("DocBin")?;
            let bin = cls.call((), None)?;
            let bin = bin.call_method1("from_disk", (path,))?;
            Ok(DocBin { obj: bin.into() })
        })
    }

    /// Recover `Doc` objects from the bin using the given `Vocab`.
    pub fn get_docs(&self, vocab: &Vocab) -> Result<Vec<Doc>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let vocab = vocab.obj.bind(py);
            let docs = obj.call_method1("get_docs", (vocab,))?;
            let mut result = Vec::new();
            for doc in docs.iter()? {
                result.push(Doc::new(doc?.into()));
            }
            Ok(result)
        })
    }
}
