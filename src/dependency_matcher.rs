#![allow(non_local_definitions)]

use crate::{utils::with_gil, Doc, SpaCyError, Vocab};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

/// A single node in a `DependencyMatcher` pattern.
#[derive(Debug, Clone)]
pub struct DependencyPatternNode {
    /// The name of the left-hand node (None for the anchor node).
    pub left_id: Option<String>,
    /// The relation operator (None for the anchor node).
    pub rel_op: Option<String>,
    /// A unique name for the right-hand node.
    pub right_id: String,
    /// Token attributes to match for the right-hand node.
    pub right_attrs: crate::matcher::TokenPattern,
}

impl Serialize for DependencyPatternNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if let Some(left_id) = &self.left_id {
            map.serialize_entry("LEFT_ID", left_id)?;
        }
        if let Some(rel_op) = &self.rel_op {
            map.serialize_entry("REL_OP", rel_op)?;
        }
        map.serialize_entry("RIGHT_ID", &self.right_id)?;
        map.serialize_entry("RIGHT_ATTRS", &self.right_attrs)?;
        map.end()
    }
}

/// A match result from `DependencyMatcher`.
#[derive(Debug, Clone)]
pub struct DependencyMatch {
    pub id: u64,
    pub token_indices: Vec<usize>,
}

/// spaCy's dependency-tree matcher.
#[derive(Debug, Clone)]
pub struct DependencyMatcher {
    pub(crate) obj: Py<PyAny>,
}

impl DependencyMatcher {
    pub fn new(vocab: &Vocab, validate: bool) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py
                .import_bound("spacy.matcher")?
                .getattr("DependencyMatcher")?;
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("validate", validate)?;
            let matcher = cls.call((vocab.obj.bind(py),), Some(&kwargs))?;
            Ok(DependencyMatcher {
                obj: matcher.into(),
            })
        })
    }

    pub fn add(
        &self,
        name: &str,
        patterns: Vec<Vec<DependencyPatternNode>>,
    ) -> Result<(), SpaCyError> {
        let json = serde_json::to_string(&patterns)?;
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let patterns_py = py.import_bound("json")?.getattr("loads")?.call1((json,))?;
            obj.call_method1("add", (name, patterns_py))?;
            Ok(())
        })
    }

    pub fn call(&self, doc: &Doc) -> Result<Vec<DependencyMatch>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = doc.obj.bind(py);
            let matches = obj.call1((doc,))?;
            let mut result = Vec::new();
            for m in matches.iter()? {
                let m = m?;
                let tuple = match m.downcast::<pyo3::types::PyTuple>() {
                    Ok(t) => t,
                    Err(e) => return Err(SpaCyError::Python(format!("Downcast error: {}", e))),
                };
                let id: u64 = tuple.get_item(0)?.extract()?;
                let indices = tuple.get_item(1)?;
                let mut token_indices = Vec::new();
                for idx in indices.iter()? {
                    token_indices.push(idx?.extract()?);
                }
                result.push(DependencyMatch { id, token_indices });
            }
            Ok(result)
        })
    }

    pub fn len(&self) -> Result<usize, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let len: usize = obj.call_method0("__len__")?.extract()?;
            Ok(len)
        })
    }

    pub fn is_empty(&self) -> Result<bool, SpaCyError> {
        Ok(self.len()? == 0)
    }

    pub fn contains(&self, name: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let contains: bool = obj.call_method1("__contains__", (name,))?.extract()?;
            Ok(contains)
        })
    }

    pub fn remove(&self, name: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("remove", (name,))?;
            Ok(())
        })
    }
}
