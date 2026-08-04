use crate::utils::{extract_vec_f32, with_gil};
use crate::{Doc, SpaCyError, Span};
use pyo3::prelude::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) obj: Py<PyAny>,
}

impl Token {
    pub(crate) fn new(obj: Py<PyAny>) -> Self {
        Token { obj }
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

    fn get_tokens(&self, name: &str) -> Result<Vec<Token>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let iter = obj.getattr(name)?;
            let mut tokens = Vec::new();
            for token in iter.iter()? {
                tokens.push(Token::new(token?.into()));
            }
            Ok(tokens)
        })
    }

    // Text & morphology
    pub fn text(&self) -> Result<String, SpaCyError> {
        self.get_attr("text")
    }
    pub fn orth_(&self) -> Result<String, SpaCyError> {
        self.get_attr("orth_")
    }
    pub fn lemma_(&self) -> Result<String, SpaCyError> {
        self.get_attr("lemma_")
    }
    pub fn norm_(&self) -> Result<String, SpaCyError> {
        self.get_attr("norm_")
    }
    pub fn lower_(&self) -> Result<String, SpaCyError> {
        self.get_attr("lower_")
    }
    pub fn shape_(&self) -> Result<String, SpaCyError> {
        self.get_attr("shape_")
    }
    pub fn prefix_(&self) -> Result<String, SpaCyError> {
        self.get_attr("prefix_")
    }
    pub fn suffix_(&self) -> Result<String, SpaCyError> {
        self.get_attr("suffix_")
    }
    pub fn morph_(&self) -> Result<String, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let morph = obj.getattr("morph")?;
            let s = morph.str()?;
            let s: String = s.extract()?;
            Ok(s)
        })
    }
    pub fn whitespace_(&self) -> Result<String, SpaCyError> {
        self.get_attr("whitespace_")
    }

    // POS & Dependencies
    pub fn pos_(&self) -> Result<String, SpaCyError> {
        self.get_attr("pos_")
    }
    pub fn tag_(&self) -> Result<String, SpaCyError> {
        self.get_attr("tag_")
    }
    pub fn dep_(&self) -> Result<String, SpaCyError> {
        self.get_attr("dep_")
    }

    pub fn head(&self) -> Result<Token, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let head = obj.getattr("head")?;
            Ok(Token::new(head.into()))
        })
    }

    pub fn children(&self) -> Result<Vec<Token>, SpaCyError> {
        self.get_tokens("children")
    }
    pub fn lefts(&self) -> Result<Vec<Token>, SpaCyError> {
        self.get_tokens("lefts")
    }
    pub fn rights(&self) -> Result<Vec<Token>, SpaCyError> {
        self.get_tokens("rights")
    }
    pub fn ancestors(&self) -> Result<Vec<Token>, SpaCyError> {
        self.get_tokens("ancestors")
    }
    pub fn subtree(&self) -> Result<Vec<Token>, SpaCyError> {
        self.get_tokens("subtree")
    }

    pub fn nbor(&self, offset: i64) -> Result<Token, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let nbor = obj.call_method1("nbor", (offset,))?;
            Ok(Token::new(nbor.into()))
        })
    }

    // Entities
    pub fn ent_type_(&self) -> Result<String, SpaCyError> {
        self.get_attr("ent_type_")
    }
    pub fn ent_iob_(&self) -> Result<String, SpaCyError> {
        self.get_attr("ent_iob_")
    }
    pub fn ent_kb_id_(&self) -> Result<String, SpaCyError> {
        self.get_attr("ent_kb_id_")
    }
    pub fn ent_id_(&self) -> Result<String, SpaCyError> {
        self.get_attr("ent_id_")
    }

    // Lexeme features
    pub fn rank(&self) -> Result<u64, SpaCyError> {
        self.get_attr("rank")
    }
    pub fn prob(&self) -> Result<f64, SpaCyError> {
        self.get_attr("prob")
    }
    pub fn cluster(&self) -> Result<u64, SpaCyError> {
        self.get_attr("cluster")
    }

    // Booleans
    pub fn is_alpha(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_alpha")
    }
    pub fn is_ascii(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_ascii")
    }
    pub fn is_digit(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_digit")
    }
    pub fn is_lower(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_lower")
    }
    pub fn is_upper(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_upper")
    }
    pub fn is_title(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_title")
    }
    pub fn is_punct(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_punct")
    }
    pub fn is_space(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_space")
    }
    pub fn is_stop(&self) -> Result<bool, SpaCyError> {
        self.get_attr("is_stop")
    }
    pub fn like_num(&self) -> Result<bool, SpaCyError> {
        self.get_attr("like_num")
    }
    pub fn like_email(&self) -> Result<bool, SpaCyError> {
        self.get_attr("like_email")
    }
    pub fn like_url(&self) -> Result<bool, SpaCyError> {
        self.get_attr("like_url")
    }

    // Vectors & context
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

    pub fn similarity(&self, other: &Token) -> Result<f64, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let other = other.obj.bind(py);
            let sim = obj.call_method1("similarity", (other,))?;
            let val: f64 = sim.extract()?;
            Ok(val)
        })
    }

    pub fn vector_norm(&self) -> Result<f64, SpaCyError> {
        self.get_attr("vector_norm")
    }

    // Position & scope
    pub fn idx(&self) -> Result<usize, SpaCyError> {
        self.get_attr("idx")
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
}
