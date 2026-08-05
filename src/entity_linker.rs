use crate::{utils::with_gil, Doc, KnowledgeBase, Language, SpaCyError};
use pyo3::prelude::*;

/// spaCy pipeline component for entity linking.
#[derive(Debug, Clone)]
pub struct EntityLinker {
    pub(crate) obj: Py<PyAny>,
}

impl EntityLinker {
    /// Create a new `EntityLinker` and add it to a `Language` pipeline.
    ///
    /// * `entity_vector_length` — dimensionality of entity vectors in the KB.
    pub fn new(
        language: &Language,
        name: &str,
        entity_vector_length: usize,
    ) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let obj = language.obj.bind(py);
            let kwargs = pyo3::types::PyDict::new_bound(py);
            kwargs.set_item("name", name)?;
            kwargs.set_item("config", pyo3::types::PyDict::new_bound(py))?;
            kwargs
                .get_item("config")?
                .unwrap()
                .set_item("entity_vector_length", entity_vector_length)?;
            obj.call_method("add_pipe", ("entity_linker",), Some(&kwargs))?;
            let el = obj.getattr("get_pipe")?.call1((name,))?;
            Ok(EntityLinker { obj: el.into() })
        })
    }

    /// Retrieve an existing `EntityLinker` from a `Language` pipeline by name.
    pub fn from_pipe(language: &Language, name: &str) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let obj = language.obj.bind(py);
            let el = obj.getattr("get_pipe")?.call1((name,))?;
            Ok(EntityLinker { obj: el.into() })
        })
    }

    /// Set the knowledge base for this linker.
    ///
    /// spaCy expects a callable `kb_loader(vocab) -> KnowledgeBase`.
    pub fn set_kb(&self, kb: &KnowledgeBase) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let kb_ref = kb.obj.bind(py);
            // Build a lambda that ignores vocab and returns our KB
            let globals = pyo3::types::PyDict::new_bound(py);
            globals.set_item("_kb", kb_ref)?;
            let loader = py.eval_bound("lambda vocab: _kb", Some(&globals), None)?;
            obj.call_method1("set_kb", (loader,))?;
            Ok(())
        })
    }

    /// Process a `Doc` and add entity links.
    ///
    /// **Note:** the pipeline (or at least this component) must be initialized
    /// before calling.
    pub fn call(&self, doc: &Doc) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = doc.obj.bind(py);
            let result = obj.call1((doc,))?;
            Ok(Doc::new(result.into()))
        })
    }

    /// Labels this linker is configured to handle.
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

    /// Component configuration.
    pub fn cfg(&self) -> Result<serde_json::Value, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let cfg = obj.getattr("cfg")?;
            crate::utils::pyobject_to_json(&cfg)
        })
    }

    /// Component name.
    pub fn name(&self) -> Result<String, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let name: String = obj.getattr("name")?.extract()?;
            Ok(name)
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let bytes: Vec<u8> = obj.call_method0("to_bytes")?.extract()?;
            Ok(bytes)
        })
    }

    pub fn from_bytes(language: &Language, name: &str, bytes: &[u8]) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let obj = language.obj.bind(py);
            let el = obj.getattr("get_pipe")?.call1((name,))?;
            el.call_method1("from_bytes", (bytes,))?;
            Ok(EntityLinker { obj: el.into() })
        })
    }

    pub fn to_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("to_disk", (path,))?;
            Ok(())
        })
    }

    pub fn from_disk(language: &Language, name: &str, path: &str) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let obj = language.obj.bind(py);
            let el = obj.getattr("get_pipe")?.call1((name,))?;
            el.call_method1("from_disk", (path,))?;
            Ok(EntityLinker { obj: el.into() })
        })
    }
}
