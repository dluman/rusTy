use crate::utils::{extract_vec_f32, with_gil};
use crate::{Doc, SpaCyError, Token};
use pyo3::prelude::*;

#[derive(Debug, Clone)]
pub struct Span {
    pub(crate) obj: Py<PyAny>,
}

impl Span {
    pub(crate) fn new(obj: Py<PyAny>) -> Self {
        Span { obj }
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
    pub fn start(&self) -> Result<usize, SpaCyError> {
        self.get_attr("start")
    }
    pub fn end(&self) -> Result<usize, SpaCyError> {
        self.get_attr("end")
    }
    pub fn start_char(&self) -> Result<usize, SpaCyError> {
        self.get_attr("start_char")
    }
    pub fn end_char(&self) -> Result<usize, SpaCyError> {
        self.get_attr("end_char")
    }
    pub fn label_(&self) -> Result<String, SpaCyError> {
        self.get_attr("label_")
    }
    pub fn label(&self) -> Result<i64, SpaCyError> {
        self.get_attr("label")
    }
    pub fn kb_id_(&self) -> Result<String, SpaCyError> {
        self.get_attr("kb_id_")
    }

    pub fn tokens(&self) -> Result<Vec<Token>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let mut tokens = Vec::new();
            for token in obj.iter()? {
                tokens.push(Token::new(token?.into()));
            }
            Ok(tokens)
        })
    }

    pub fn root(&self) -> Result<Token, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let root = obj.getattr("root")?;
            Ok(Token::new(root.into()))
        })
    }

    pub fn sent(&self) -> Result<Span, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let sent = obj.getattr("sent")?;
            Ok(Span::new(sent.into()))
        })
    }

    pub fn doc(&self) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = obj.getattr("doc")?;
            Ok(Doc::new(doc.into()))
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

    pub fn vector_norm(&self) -> Result<f64, SpaCyError> {
        self.get_attr("vector_norm")
    }

    pub fn similarity(&self, other: &Span) -> Result<f64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let other = other.obj.bind(py);
            let sim = obj.call_method1("similarity", (other,))?;
            let val: f64 = sim.extract()?;
            Ok(val)
        })
    }

    pub fn as_doc(&self) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = obj.call_method0("as_doc")?;
            Ok(Doc::new(doc.into()))
        })
    }
}
