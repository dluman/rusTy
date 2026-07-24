use pyo3::prelude::*;
use crate::{SpaCyError, Token, Span, Language};
use crate::utils::{with_gil, extract_vec_f32};
use serde_json::Value;

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

    pub fn to_json(&self) -> Result<Value, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let json_str = obj.call_method0("to_json")?;
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
            doc.call_method1("from_bytes", (bytes,))?;
            Ok(Doc::new(doc.into()))
        })
    }
}
