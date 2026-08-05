#![allow(non_local_definitions)]

use crate::{utils::with_gil, Doc, Language, SpaCyError, TokenPattern};
use pyo3::prelude::*;
use serde::Serialize;

/// A phrase or token-based pattern for the `EntityRuler`.
#[derive(Debug, Clone, Serialize)]
pub struct EntityPattern {
    pub label: String,
    pub pattern: EntityPatternValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl EntityPattern {
    pub fn phrase(label: impl Into<String>, pattern: impl Into<String>) -> Self {
        EntityPattern {
            label: label.into(),
            pattern: EntityPatternValue::Phrase(pattern.into()),
            id: None,
        }
    }

    pub fn tokens(label: impl Into<String>, patterns: Vec<TokenPattern>) -> Self {
        EntityPattern {
            label: label.into(),
            pattern: EntityPatternValue::Tokens(patterns),
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
pub enum EntityPatternValue {
    Phrase(String),
    Tokens(Vec<TokenPattern>),
}

/// spaCy pipeline component for rule-based named entity recognition.
#[derive(Debug, Clone)]
pub struct EntityRuler {
    pub(crate) obj: Py<PyAny>,
}

impl EntityRuler {
    /// Create a new `EntityRuler`.
    ///
    /// * `phrase_matcher_attr` — optional attribute for the internal `PhraseMatcher`
    ///   (e.g. `"LOWER"`).
    /// * `validate` — validate patterns before adding.
    /// * `overwrite_ents` — overwrite existing entities with matches.
    /// * `patterns` — optional initial patterns.
    pub fn new(
        language: &Language,
        phrase_matcher_attr: Option<&str>,
        validate: bool,
        overwrite_ents: bool,
        patterns: Option<&[EntityPattern]>,
    ) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.pipeline")?.getattr("EntityRuler")?;
            let kwargs = pyo3::types::PyDict::new_bound(py);
            if let Some(attr) = phrase_matcher_attr {
                kwargs.set_item("phrase_matcher_attr", attr)?;
            }
            kwargs.set_item("validate", validate)?;
            kwargs.set_item("overwrite_ents", overwrite_ents)?;
            if let Some(patterns) = patterns {
                let json = serde_json::to_string(patterns)?;
                let patterns_py = py.import_bound("json")?.getattr("loads")?.call1((json,))?;
                kwargs.set_item("patterns", patterns_py)?;
            }
            let ruler = cls.call((language.obj.bind(py),), Some(&kwargs))?;
            Ok(EntityRuler { obj: ruler.into() })
        })
    }

    /// Add patterns to the ruler.
    pub fn add_patterns(&self, patterns: &[EntityPattern]) -> Result<(), SpaCyError> {
        let json = serde_json::to_string(patterns)?;
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let patterns_py = py.import_bound("json")?.getattr("loads")?.call1((json,))?;
            obj.call_method1("add_patterns", (patterns_py,))?;
            Ok(())
        })
    }

    /// Add patterns from a raw JSON string.
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

    /// Process a `Doc` and add any entity matches to `doc.ents`.
    pub fn call(&self, doc: &Doc) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = doc.obj.bind(py);
            let result = obj.call1((doc,))?;
            Ok(Doc::new(result.into()))
        })
    }

    /// All labels present in the match patterns.
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

    /// All entity IDs present in the patterns.
    pub fn ent_ids(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let ids = obj.getattr("ent_ids")?;
            let mut result = Vec::new();
            for id in ids.iter()? {
                result.push(id?.extract()?);
            }
            Ok(result)
        })
    }

    /// Get all patterns that were added.
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

    /// Check if a label is present.
    pub fn contains(&self, label: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let contains: bool = obj.call_method1("__contains__", (label,))?.extract()?;
            Ok(contains)
        })
    }

    /// Total number of patterns.
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

    /// Serialize patterns to bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let bytes: Vec<u8> = obj.call_method0("to_bytes")?.extract()?;
            Ok(bytes)
        })
    }

    /// Load patterns from bytes (modifies in place).
    pub fn from_bytes(&self, bytes: &[u8]) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("from_bytes", (bytes,))?;
            Ok(())
        })
    }

    /// Save patterns to disk (JSONL + cfg).
    pub fn to_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("to_disk", (path,))?;
            Ok(())
        })
    }

    /// Load patterns from disk (modifies in place).
    pub fn from_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("from_disk", (path,))?;
            Ok(())
        })
    }
}
