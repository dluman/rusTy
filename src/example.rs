use crate::{utils::with_gil, Doc, Language, SpaCyError};
use pyo3::prelude::*;

/// A spaCy training `Example`: pairs a predicted `Doc` with a reference `Doc`.
#[derive(Debug, Clone)]
pub struct Example {
    pub(crate) obj: Py<PyAny>,
}

impl Example {
    /// Create an `Example` from a `Doc` and an annotation dictionary.
    pub fn from_dict(doc: &Doc, annotations: &str) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.training")?.getattr("Example")?;
            let doc = doc.obj.bind(py);
            let ann_dict = py
                .import_bound("json")?
                .getattr("loads")?
                .call1((annotations,))?;
            let example = cls.call_method1("from_dict", (doc, ann_dict))?;
            Ok(Example {
                obj: example.into(),
            })
        })
    }

    /// Create an `Example` from a `Language` and a text/annotation pair.
    pub fn from_text_and_annotations(
        language: &Language,
        text: &str,
        annotations: &str,
    ) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.training")?.getattr("Example")?;
            let doc = language.obj.bind(py).call_method1("make_doc", (text,))?;
            let ann_dict = py
                .import_bound("json")?
                .getattr("loads")?
                .call1((annotations,))?;
            let example = cls.call_method1("from_dict", (doc, ann_dict))?;
            Ok(Example {
                obj: example.into(),
            })
        })
    }

    /// Predicted document (`Doc` before training).
    pub fn predicted(&self) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = obj.getattr("predicted")?;
            Ok(Doc::new(doc.into()))
        })
    }

    /// Reference document (`Doc` with gold annotations).
    pub fn reference(&self) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = obj.getattr("reference")?;
            Ok(Doc::new(doc.into()))
        })
    }

    /// Text of the example.
    pub fn text(&self) -> Result<String, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let text: String = obj.getattr("text")?.extract()?;
            Ok(text)
        })
    }

    /// Convert to a spaCy annotation dictionary.
    pub fn to_dict(&self) -> Result<serde_json::Value, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let dict = obj.call_method0("to_dict")?;
            crate::utils::pyobject_to_json(&dict)
        })
    }

    /// Get aligned NER tags from the reference.
    pub fn get_aligned_ner(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let tags = obj.call_method0("get_aligned_ner")?;
            let mut result = Vec::new();
            for tag in tags.iter()? {
                result.push(tag?.extract()?);
            }
            Ok(result)
        })
    }

    /// Split the example into one example per sentence.
    pub fn split_sents(&self) -> Result<Vec<Example>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let examples = obj.call_method0("split_sents")?;
            let mut result = Vec::new();
            for ex in examples.iter()? {
                result.push(Example { obj: ex?.into() });
            }
            Ok(result)
        })
    }
}
