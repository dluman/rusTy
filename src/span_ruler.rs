#![allow(non_local_definitions)]

use crate::{utils::with_gil, Doc, Language, SpaCyError, TokenPattern};
use pyo3::prelude::*;
use serde::Serialize;

/// A phrase or token-based pattern for the `SpanRuler`.
#[derive(Debug, Clone, Serialize)]
pub struct SpanPattern {
    pub label: String,
    pub pattern: SpanPatternValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl SpanPattern {
    pub fn phrase(label: impl Into<String>, pattern: impl Into<String>) -> Self {
        SpanPattern {
            label: label.into(),
            pattern: SpanPatternValue::Phrase(pattern.into()),
            id: None,
        }
    }

    pub fn tokens(label: impl Into<String>, patterns: Vec<TokenPattern>) -> Self {
        SpanPattern {
            label: label.into(),
            pattern: SpanPatternValue::Tokens(patterns),
            id: None,
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SpanPatternValue {
    Phrase(String),
    Tokens(Vec<TokenPattern>),
}

/// spaCy pipeline component for rule-based span recognition.
#[derive(Debug, Clone)]
pub struct SpanRuler {
    pub(crate) obj: Py<PyAny>,
}

impl SpanRuler {
    pub fn new(
        language: &Language,
        spans_key: Option<&str>,
        annotate_ents: bool,
        validate: bool,
        overwrite: bool,
        patterns: Option<&[SpanPattern]>,
    ) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.pipeline")?.getattr("SpanRuler")?;
            let kwargs = pyo3::types::PyDict::new_bound(py);
            if let Some(key) = spans_key {
                kwargs.set_item("spans_key", key)?;
            }
            kwargs.set_item("annotate_ents", annotate_ents)?;
            kwargs.set_item("validate", validate)?;
            kwargs.set_item("overwrite", overwrite)?;
            if let Some(patterns) = patterns {
                let json = serde_json::to_string(patterns)?;
                let patterns_py = py.import_bound("json")?.getattr("loads")?.call1((json,))?;
                kwargs.set_item("patterns", patterns_py)?;
            }
            let ruler = cls.call((language.obj.bind(py),), Some(&kwargs))?;
            Ok(SpanRuler { obj: ruler.into() })
        })
    }

    pub fn add_patterns(&self, patterns: &[SpanPattern]) -> Result<(), SpaCyError> {
        let json = serde_json::to_string(patterns)?;
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let patterns_py = py.import_bound("json")?.getattr("loads")?.call1((json,))?;
            obj.call_method1("add_patterns", (patterns_py,))?;
            Ok(())
        })
    }

    pub fn add_patterns_raw(&self, patterns_json: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let patterns_py = py
                .import_bound("json")?
                .getattr("loads")?
                .call1((patterns_json,))?;
            obj.call_method1("add_patterns", (patterns_py,))?;
            Ok(())
        })
    }

    pub fn call(&self, doc: &Doc) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = doc.obj.bind(py);
            let result = obj.call1((doc,))?;
            Ok(Doc::new(result.into()))
        })
    }

    pub fn labels(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let labels = obj.getattr("labels")?;
            let mut result = Vec::new();
            for label in labels.iter()? {
                result.push(label?.extract()?);
            }
            Ok(result)
        })
    }

    pub fn ids(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let ids = obj.getattr("ids")?;
            let mut result = Vec::new();
            for id in ids.iter()? {
                result.push(id?.extract()?);
            }
            Ok(result)
        })
    }

    pub fn patterns(&self) -> Result<Vec<serde_json::Value>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let patterns = obj.getattr("patterns")?;
            let json_mod = py.import_bound("json")?;
            let dumps = json_mod.getattr("dumps")?;
            let mut result = Vec::new();
            for pattern in patterns.iter()? {
                let pattern = pattern?;
                let json_str: String = dumps.call1((pattern,))?.extract()?;
                let value = serde_json::from_str(&json_str)?;
                result.push(value);
            }
            Ok(result)
        })
    }

    pub fn contains(&self, label: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let contains: bool = obj.call_method1("__contains__", (label,))?.extract()?;
            Ok(contains)
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

    pub fn remove(&self, label: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("remove", (label,))?;
            Ok(())
        })
    }

    pub fn remove_by_id(&self, id: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("remove_by_id", (id,))?;
            Ok(())
        })
    }

    pub fn clear(&self) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method0("clear")?;
            Ok(())
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let bytes: Vec<u8> = obj.call_method0("to_bytes")?.extract()?;
            Ok(bytes)
        })
    }

    pub fn from_bytes(&self, bytes: &[u8]) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("from_bytes", (bytes,))?;
            Ok(())
        })
    }

    pub fn to_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("to_disk", (path,))?;
            Ok(())
        })
    }

    pub fn from_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("from_disk", (path,))?;
            Ok(())
        })
    }
}
