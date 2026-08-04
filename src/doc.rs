use crate::extensions::{
    py_call_underscore, py_get_underscore, py_has_extension, py_has_underscore,
    py_remove_extension, py_set_extension, py_set_underscore, ExtensionDefinition, ExtensionInfo,
};
use crate::utils::{extract_vec_f32, with_gil};
use crate::{Language, RetokenizerGuard, SpaCyError, Span, SpanGroups, Token, Vocab};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyDictMethods};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Doc {
    pub(crate) obj: Py<PyAny>,
}

impl Doc {
    pub(crate) fn new(obj: Py<PyAny>) -> Self {
        Doc { obj }
    }

    fn get_attr<T>(&self, name: &str) -> Result<T, SpaCyError>
    where
        T: for<'a> FromPyObject<'a>,
    {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let val: T = obj.getattr(name)?.extract()?;
            Ok(val)
        })
    }

    pub fn text(&self) -> Result<String, SpaCyError> {
        self.get_attr("text")
    }

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

    pub fn token(&self, i: usize) -> Result<Token, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let token = obj.call_method1("__getitem__", (i,))?;
            Ok(Token::new(token.into()))
        })
    }

    pub fn tokens(&self) -> Result<Vec<Token>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let len = obj.len()?;
            let mut tokens = Vec::with_capacity(len);
            for i in 0..len {
                let token = obj.call_method1("__getitem__", (i,))?;
                tokens.push(Token::new(token.into()));
            }
            Ok(tokens)
        })
    }

    pub fn ents(&self) -> Result<Vec<Span>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let ents = obj.getattr("ents")?;
            let mut spans = Vec::new();
            for ent in ents.iter()? {
                spans.push(Span::new(ent?.into()));
            }
            Ok(spans)
        })
    }

    pub fn sents(&self) -> Result<Vec<Span>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let sents = obj.getattr("sents")?;
            let mut spans = Vec::new();
            for sent in sents.iter()? {
                spans.push(Span::new(sent?.into()));
            }
            Ok(spans)
        })
    }

    pub fn noun_chunks(&self) -> Result<Vec<Span>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let chunks = obj.getattr("noun_chunks")?;
            let mut spans = Vec::new();
            for chunk in chunks.iter()? {
                spans.push(Span::new(chunk?.into()));
            }
            Ok(spans)
        })
    }

    pub fn char_span(
        &self,
        start_char: usize,
        end_char: usize,
        label: Option<&str>,
        kb_id: Option<&str>,
        alignment_mode: Option<&str>,
    ) -> Result<Option<Span>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            if let Some(lbl) = label {
                kwargs.set_item("label", lbl)?;
            }
            if let Some(id) = kb_id {
                kwargs.set_item("kb_id", id)?;
            }
            if let Some(mode) = alignment_mode {
                kwargs.set_item("alignment_mode", mode)?;
            }
            let span = obj.call_method("char_span", (start_char, end_char), Some(&kwargs))?;
            if span.is_none() {
                Ok(None)
            } else {
                Ok(Some(Span::new(span.into())))
            }
        })
    }

    pub fn vector(&self) -> Result<Vec<f32>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let vec = obj.getattr("vector")?;
            extract_vec_f32(&vec)
        })
    }

    pub fn has_vector(&self) -> Result<bool, SpaCyError> {
        self.get_attr("has_vector")
    }

    pub fn similarity(&self, other: &Doc) -> Result<f64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let other = other.obj.bind(py);
            let sim = obj.call_method1("similarity", (other,))?;
            let val: f64 = sim.extract()?;
            Ok(val)
        })
    }

    pub fn lang(&self) -> Result<String, SpaCyError> {
        self.get_attr("lang")
    }

    pub fn vocab(&self) -> Result<Vocab, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let vocab = obj.getattr("vocab")?;
            Ok(Vocab::new(vocab.into()))
        })
    }

    pub fn retokenize(&self) -> Result<RetokenizerGuard, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let retokenizer = obj.call_method0("retokenize")?;
            retokenizer.call_method0("__enter__")?;
            Ok(RetokenizerGuard::new(retokenizer.into()))
        })
    }

    pub fn spans(&self) -> Result<SpanGroups, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let spans = obj.getattr("spans")?;
            Ok(SpanGroups {
                obj: spans.into(),
                doc: self.obj.clone(),
            })
        })
    }

    pub fn cats(&self) -> Result<HashMap<String, f64>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let cats = obj.getattr("cats")?;
            let items = cats.call_method0("items")?;
            let mut result = HashMap::new();
            for item in items.iter()? {
                let item = item?;
                let key: String = item.call_method1("__getitem__", (0,))?.extract()?;
                let value: f64 = item.call_method1("__getitem__", (1,))?.extract()?;
                result.insert(key, value);
            }
            Ok(result)
        })
    }

    pub fn vector_norm(&self) -> Result<f64, SpaCyError> {
        self.get_attr("vector_norm")
    }

    pub fn has_annotation(&self, attr: &str, require_complete: bool) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("require_complete", require_complete)?;
            let result: bool = obj
                .call_method("has_annotation", (attr,), Some(&kwargs))?
                .extract()?;
            Ok(result)
        })
    }

    pub fn count_by(&self, attr_id: u64) -> Result<HashMap<u64, i64>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let counts = obj.call_method1("count_by", (attr_id,))?;
            let items = counts.call_method0("items")?;
            let mut result = HashMap::new();
            for item in items.iter()? {
                let item = item?;
                let key: u64 = item.call_method1("__getitem__", (0,))?.extract()?;
                let value: i64 = item.call_method1("__getitem__", (1,))?.extract()?;
                result.insert(key, value);
            }
            Ok(result)
        })
    }

    pub fn to_json(&self) -> Result<Value, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let json_dict = obj.call_method0("to_json")?;
            // spaCy returns a Python dict; convert it to a JSON string via json.dumps
            let json_str = py
                .import_bound("json")?
                .getattr("dumps")?
                .call1((json_dict,))?;
            let json_str: String = json_str.extract()?;
            let value = serde_json::from_str(&json_str)?;
            Ok(value)
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let bytes = obj.call_method0("to_bytes")?;
            let bytes: Vec<u8> = bytes.extract()?;
            Ok(bytes)
        })
    }

    pub fn from_bytes(lang: &Language, bytes: &[u8]) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let vocab = lang.obj.bind(py).getattr("vocab")?;
            let tokens_mod = py.import_bound("spacy.tokens")?;
            let doc_cls = tokens_mod.getattr("Doc")?;
            let doc = doc_cls.call1((vocab,))?;
            let doc = doc.call_method1("from_bytes", (bytes,))?;
            Ok(Doc::new(doc.into()))
        })
    }

    pub fn to_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("to_disk", (path,))?;
            Ok(())
        })
    }

    pub fn from_disk(lang: &Language, path: &str) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let vocab = lang.obj.bind(py).getattr("vocab")?;
            let tokens_mod = py.import_bound("spacy.tokens")?;
            let doc_cls = tokens_mod.getattr("Doc")?;
            let doc = doc_cls.call1((vocab,))?;
            let doc = doc.call_method1("from_disk", (path,))?;
            Ok(Doc::new(doc.into()))
        })
    }

    pub fn from_json(lang: &Language, json: &serde_json::Value) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let vocab = lang.obj.bind(py).getattr("vocab")?;
            let tokens_mod = py.import_bound("spacy.tokens")?;
            let doc_cls = tokens_mod.getattr("Doc")?;
            let doc = doc_cls.call1((vocab,))?;
            let json_str = serde_json::to_string(json)?;
            let json_py = py
                .import_bound("json")?
                .getattr("loads")?
                .call1((json_str,))?;
            let doc = doc.call_method("from_json", (json_py,), None)?;
            Ok(Doc::new(doc.into()))
        })
    }

    // Extension class methods
    pub fn set_extension(
        name: &str,
        def: ExtensionDefinition,
        force: bool,
    ) -> Result<(), SpaCyError> {
        with_gil(|py| py_set_extension(py, "Doc", name, &def, force))
    }

    pub fn has_extension(name: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| py_has_extension(py, "Doc", name))
    }

    pub fn remove_extension(name: &str) -> Result<ExtensionInfo, SpaCyError> {
        with_gil(|py| py_remove_extension(py, "Doc", name))
    }

    // Instance access to ._ namespace
    pub fn get_underscore(&self, name: &str) -> Result<serde_json::Value, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            py_get_underscore(obj, name)
        })
    }

    pub fn set_underscore(&self, name: &str, value: serde_json::Value) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            py_set_underscore(obj, name, value)
        })
    }

    pub fn has_underscore(&self, name: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            py_has_underscore(obj, name)
        })
    }

    pub fn call_underscore(
        &self,
        name: &str,
        args: &[serde_json::Value],
        kwargs: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            py_call_underscore(obj, name, args, kwargs)
        })
    }
}
