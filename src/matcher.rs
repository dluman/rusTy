#![allow(non_local_definitions)]

use crate::{utils::with_gil, Doc, SpaCyError, Span, Vocab};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;
use std::collections::HashMap;

/// A single token's value within a matcher pattern.
/// Supports exact values, regex, set membership, fuzzy matching,
/// subset/superset/intersection, and comparison operators.
#[derive(Debug, Clone)]
pub enum PatternValue<T> {
    Exact(T),
    Regex(String),
    In(Vec<T>),
    NotIn(Vec<T>),
    Fuzzy(String),
    FuzzyN(u8, String),
    IsSubset(Vec<T>),
    IsSuperset(Vec<T>),
    Intersects(Vec<T>),
    Eq(T),
    Gte(T),
    Lte(T),
    Gt(T),
    Lt(T),
}

impl<T> PatternValue<T> {
    pub fn exact(value: T) -> Self {
        PatternValue::Exact(value)
    }
    pub fn regex(value: impl Into<String>) -> Self {
        PatternValue::Regex(value.into())
    }
    pub fn in_list(values: Vec<T>) -> Self {
        PatternValue::In(values)
    }
    pub fn not_in(values: Vec<T>) -> Self {
        PatternValue::NotIn(values)
    }
    pub fn fuzzy(value: impl Into<String>) -> Self {
        PatternValue::Fuzzy(value.into())
    }
    pub fn fuzzy_n(n: u8, value: impl Into<String>) -> Self {
        PatternValue::FuzzyN(n, value.into())
    }
    pub fn is_subset(values: Vec<T>) -> Self {
        PatternValue::IsSubset(values)
    }
    pub fn is_superset(values: Vec<T>) -> Self {
        PatternValue::IsSuperset(values)
    }
    pub fn intersects(values: Vec<T>) -> Self {
        PatternValue::Intersects(values)
    }
    pub fn eq(value: T) -> Self {
        PatternValue::Eq(value)
    }
    pub fn gte(value: T) -> Self {
        PatternValue::Gte(value)
    }
    pub fn lte(value: T) -> Self {
        PatternValue::Lte(value)
    }
    pub fn gt(value: T) -> Self {
        PatternValue::Gt(value)
    }
    pub fn lt(value: T) -> Self {
        PatternValue::Lt(value)
    }
}

impl From<&str> for PatternValue<String> {
    fn from(value: &str) -> Self {
        PatternValue::Exact(value.to_string())
    }
}

impl From<String> for PatternValue<String> {
    fn from(value: String) -> Self {
        PatternValue::Exact(value)
    }
}

impl From<bool> for PatternValue<bool> {
    fn from(value: bool) -> Self {
        PatternValue::Exact(value)
    }
}

impl From<usize> for PatternValue<usize> {
    fn from(value: usize) -> Self {
        PatternValue::Exact(value)
    }
}

impl<T: Serialize> Serialize for PatternValue<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PatternValue::Exact(v) => v.serialize(serializer),
            PatternValue::Regex(s) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("REGEX", s)?;
                map.end()
            }
            PatternValue::In(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("IN", v)?;
                map.end()
            }
            PatternValue::NotIn(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("NOT_IN", v)?;
                map.end()
            }
            PatternValue::Fuzzy(s) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("FUZZY", s)?;
                map.end()
            }
            PatternValue::FuzzyN(n, s) => {
                let mut map = serializer.serialize_map(Some(1))?;
                let key = format!("FUZZY{}", n);
                map.serialize_entry(&key, s)?;
                map.end()
            }
            PatternValue::IsSubset(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("IS_SUBSET", v)?;
                map.end()
            }
            PatternValue::IsSuperset(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("IS_SUPERSET", v)?;
                map.end()
            }
            PatternValue::Intersects(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("INTERSECTS", v)?;
                map.end()
            }
            PatternValue::Eq(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("==", v)?;
                map.end()
            }
            PatternValue::Gte(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(">=", v)?;
                map.end()
            }
            PatternValue::Lte(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("<=", v)?;
                map.end()
            }
            PatternValue::Gt(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(">", v)?;
                map.end()
            }
            PatternValue::Lt(v) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("<", v)?;
                map.end()
            }
        }
    }
}

/// A single token pattern for spaCy's Matcher.
/// Constructed via the builder methods (e.g. `TokenPattern::new().orth("hello")`).
#[derive(Debug, Default, Clone, Serialize)]
pub struct TokenPattern {
    #[serde(rename = "ORTH", skip_serializing_if = "Option::is_none")]
    pub orth: Option<PatternValue<String>>,
    #[serde(rename = "TEXT", skip_serializing_if = "Option::is_none")]
    pub text: Option<PatternValue<String>>,
    #[serde(rename = "NORM", skip_serializing_if = "Option::is_none")]
    pub norm: Option<PatternValue<String>>,
    #[serde(rename = "LOWER", skip_serializing_if = "Option::is_none")]
    pub lower: Option<PatternValue<String>>,
    #[serde(rename = "SHAPE", skip_serializing_if = "Option::is_none")]
    pub shape: Option<PatternValue<String>>,
    #[serde(rename = "LEMMA", skip_serializing_if = "Option::is_none")]
    pub lemma: Option<PatternValue<String>>,
    #[serde(rename = "POS", skip_serializing_if = "Option::is_none")]
    pub pos: Option<PatternValue<String>>,
    #[serde(rename = "TAG", skip_serializing_if = "Option::is_none")]
    pub tag: Option<PatternValue<String>>,
    #[serde(rename = "MORPH", skip_serializing_if = "Option::is_none")]
    pub morph: Option<PatternValue<String>>,
    #[serde(rename = "DEP", skip_serializing_if = "Option::is_none")]
    pub dep: Option<PatternValue<String>>,
    #[serde(rename = "ENT_TYPE", skip_serializing_if = "Option::is_none")]
    pub ent_type: Option<PatternValue<String>>,
    #[serde(rename = "ENT_IOB", skip_serializing_if = "Option::is_none")]
    pub ent_iob: Option<PatternValue<String>>,
    #[serde(rename = "ENT_ID", skip_serializing_if = "Option::is_none")]
    pub ent_id: Option<PatternValue<String>>,
    #[serde(rename = "ENT_KB_ID", skip_serializing_if = "Option::is_none")]
    pub ent_kb_id: Option<PatternValue<String>>,

    #[serde(rename = "LENGTH", skip_serializing_if = "Option::is_none")]
    pub length: Option<PatternValue<usize>>,

    #[serde(rename = "IS_ALPHA", skip_serializing_if = "Option::is_none")]
    pub is_alpha: Option<PatternValue<bool>>,
    #[serde(rename = "IS_ASCII", skip_serializing_if = "Option::is_none")]
    pub is_ascii: Option<PatternValue<bool>>,
    #[serde(rename = "IS_DIGIT", skip_serializing_if = "Option::is_none")]
    pub is_digit: Option<PatternValue<bool>>,
    #[serde(rename = "IS_LOWER", skip_serializing_if = "Option::is_none")]
    pub is_lower: Option<PatternValue<bool>>,
    #[serde(rename = "IS_UPPER", skip_serializing_if = "Option::is_none")]
    pub is_upper: Option<PatternValue<bool>>,
    #[serde(rename = "IS_TITLE", skip_serializing_if = "Option::is_none")]
    pub is_title: Option<PatternValue<bool>>,
    #[serde(rename = "IS_PUNCT", skip_serializing_if = "Option::is_none")]
    pub is_punct: Option<PatternValue<bool>>,
    #[serde(rename = "IS_SPACE", skip_serializing_if = "Option::is_none")]
    pub is_space: Option<PatternValue<bool>>,
    #[serde(rename = "IS_STOP", skip_serializing_if = "Option::is_none")]
    pub is_stop: Option<PatternValue<bool>>,
    #[serde(rename = "IS_SENT_START", skip_serializing_if = "Option::is_none")]
    pub is_sent_start: Option<PatternValue<bool>>,
    #[serde(rename = "LIKE_NUM", skip_serializing_if = "Option::is_none")]
    pub like_num: Option<PatternValue<bool>>,
    #[serde(rename = "LIKE_URL", skip_serializing_if = "Option::is_none")]
    pub like_url: Option<PatternValue<bool>>,
    #[serde(rename = "LIKE_EMAIL", skip_serializing_if = "Option::is_none")]
    pub like_email: Option<PatternValue<bool>>,
    #[serde(rename = "SPACY", skip_serializing_if = "Option::is_none")]
    pub spacy: Option<PatternValue<bool>>,

    #[serde(rename = "OP", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,

    #[serde(rename = "_", skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

macro_rules! pattern_setter {
    ($name:ident, $field:ident, $ty:ty) => {
        pub fn $name(mut self, value: impl Into<$ty>) -> Self {
            self.$field = Some(value.into());
            self
        }
    };
}

impl TokenPattern {
    pub fn new() -> Self {
        Self::default()
    }

    // String-valued attributes
    pattern_setter!(orth, orth, PatternValue<String>);
    pattern_setter!(text, text, PatternValue<String>);
    pattern_setter!(norm, norm, PatternValue<String>);
    pattern_setter!(lower, lower, PatternValue<String>);
    pattern_setter!(shape, shape, PatternValue<String>);
    pattern_setter!(lemma, lemma, PatternValue<String>);
    pattern_setter!(pos, pos, PatternValue<String>);
    pattern_setter!(tag, tag, PatternValue<String>);
    pattern_setter!(morph, morph, PatternValue<String>);
    pattern_setter!(dep, dep, PatternValue<String>);
    pattern_setter!(ent_type, ent_type, PatternValue<String>);
    pattern_setter!(ent_iob, ent_iob, PatternValue<String>);
    pattern_setter!(ent_id, ent_id, PatternValue<String>);
    pattern_setter!(ent_kb_id, ent_kb_id, PatternValue<String>);

    // Numeric
    pattern_setter!(length, length, PatternValue<usize>);

    // Boolean flags
    pattern_setter!(is_alpha, is_alpha, PatternValue<bool>);
    pattern_setter!(is_ascii, is_ascii, PatternValue<bool>);
    pattern_setter!(is_digit, is_digit, PatternValue<bool>);
    pattern_setter!(is_lower, is_lower, PatternValue<bool>);
    pattern_setter!(is_upper, is_upper, PatternValue<bool>);
    pattern_setter!(is_title, is_title, PatternValue<bool>);
    pattern_setter!(is_punct, is_punct, PatternValue<bool>);
    pattern_setter!(is_space, is_space, PatternValue<bool>);
    pattern_setter!(is_stop, is_stop, PatternValue<bool>);
    pattern_setter!(is_sent_start, is_sent_start, PatternValue<bool>);
    pattern_setter!(like_num, like_num, PatternValue<bool>);
    pattern_setter!(like_url, like_url, PatternValue<bool>);
    pattern_setter!(like_email, like_email, PatternValue<bool>);
    pattern_setter!(spacy, spacy, PatternValue<bool>);

    pub fn op(mut self, value: impl Into<String>) -> Self {
        self.op = Some(value.into());
        self
    }

    pub fn custom(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom = Some(value);
        self
    }
}

/// A match result from `Matcher` or `PhraseMatcher`.
#[derive(Debug, Clone)]
pub struct Match {
    pub id: u64,
    pub start: usize,
    pub end: usize,
}

/// spaCy's rule-based `Matcher`.
#[derive(Debug, Clone)]
pub struct Matcher {
    pub(crate) obj: Py<PyAny>,
}

impl Matcher {
    pub fn new(vocab: &Vocab, validate: bool) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let matcher_cls = py.import_bound("spacy.matcher")?.getattr("Matcher")?;
            let matcher = matcher_cls.call1((vocab.obj.bind(py), validate))?;
            Ok(Matcher {
                obj: matcher.into(),
            })
        })
    }

    pub fn add(&self, name: &str, patterns: Vec<TokenPattern>) -> Result<(), SpaCyError> {
        // spaCy expects List[List[Dict]], i.e. a list of patterns.
        // We wrap the provided Vec<TokenPattern> in an outer Vec.
        let json = serde_json::to_string(&vec![patterns])?;
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let patterns_py = py.import_bound("json")?.getattr("loads")?.call1((json,))?;
            obj.call_method1("add", (name, patterns_py))?;
            Ok(())
        })
    }

    pub fn add_raw(&self, name: &str, patterns_json: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let patterns_py = py
                .import_bound("json")?
                .getattr("loads")?
                .call1((patterns_json,))?;
            obj.call_method1("add", (name, patterns_py))?;
            Ok(())
        })
    }

    pub fn call(&self, doc: &Doc) -> Result<Vec<Match>, SpaCyError> {
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
                let start: usize = tuple.get_item(1)?.extract()?;
                let end: usize = tuple.get_item(2)?.extract()?;
                result.push(Match { id, start, end });
            }
            Ok(result)
        })
    }

    pub fn call_as_spans(&self, doc: &Doc) -> Result<Vec<Span>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = doc.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("as_spans", true)?;
            let spans = obj.call_method("__call__", (doc,), Some(&kwargs))?;
            let mut result = Vec::new();
            for span in spans.iter()? {
                result.push(Span::new(span?.into()));
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

/// spaCy's `PhraseMatcher` for efficient phrase-list matching.
#[derive(Debug, Clone)]
pub struct PhraseMatcher {
    pub(crate) obj: Py<PyAny>,
}

impl PhraseMatcher {
    pub fn new(vocab: &Vocab, attr: Option<&str>, validate: bool) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let matcher_cls = py.import_bound("spacy.matcher")?.getattr("PhraseMatcher")?;
            let kwargs = PyDict::new_bound(py);
            if let Some(a) = attr {
                kwargs.set_item("attr", a)?;
            }
            kwargs.set_item("validate", validate)?;
            let matcher = matcher_cls.call((vocab.obj.bind(py),), Some(&kwargs))?;
            Ok(PhraseMatcher {
                obj: matcher.into(),
            })
        })
    }

    pub fn add(&self, name: &str, docs: &[Doc]) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let py_list = PyList::empty_bound(py);
            for doc in docs {
                py_list.append(doc.obj.bind(py))?;
            }
            obj.call_method1("add", (name, py_list))?;
            Ok(())
        })
    }

    pub fn call(&self, doc: &Doc) -> Result<Vec<Match>, SpaCyError> {
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
                let start: usize = tuple.get_item(1)?.extract()?;
                let end: usize = tuple.get_item(2)?.extract()?;
                result.push(Match { id, start, end });
            }
            Ok(result)
        })
    }

    pub fn call_as_spans(&self, doc: &Doc) -> Result<Vec<Span>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = doc.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("as_spans", true)?;
            let spans = obj.call_method("__call__", (doc,), Some(&kwargs))?;
            let mut result = Vec::new();
            for span in spans.iter()? {
                result.push(Span::new(span?.into()));
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
