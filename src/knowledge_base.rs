use crate::{utils::with_gil, SpaCyError, Vocab};
use numpy::PyArray1;
use pyo3::prelude::*;

/// A spaCy `InMemoryLookupKB` knowledge base for entity linking.
#[derive(Debug, Clone)]
pub struct KnowledgeBase {
    pub(crate) obj: Py<PyAny>,
}

impl KnowledgeBase {
    /// Create a new `InMemoryLookupKB`.
    pub fn new(vocab: &Vocab, entity_vector_length: usize) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.kb")?.getattr("InMemoryLookupKB")?;
            let kb = cls.call1((vocab.obj.bind(py), entity_vector_length))?;
            Ok(KnowledgeBase { obj: kb.into() })
        })
    }

    /// Add an entity to the knowledge base.
    pub fn add_entity(
        &self,
        entity: &str,
        freq: u32,
        entity_vector: &[f32],
    ) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let arr = PyArray1::from_vec_bound(py, entity_vector.to_vec());
            obj.call_method1("add_entity", (entity, freq, arr))?;
            Ok(())
        })
    }

    /// Add an alias (surface form) to the knowledge base.
    pub fn add_alias(
        &self,
        alias: &str,
        entities: &[&str],
        probabilities: &[f32],
    ) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let entities_py: Vec<&str> = entities.to_vec();
            let probs_py: Vec<f32> = probabilities.to_vec();
            obj.call_method1("add_alias", (alias, entities_py, probs_py))?;
            Ok(())
        })
    }

    /// Check if an entity ID exists.
    pub fn contains_entity(&self, entity: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let result: bool = obj.call_method1("contains_entity", (entity,))?.extract()?;
            Ok(result)
        })
    }

    /// Check if an alias exists.
    pub fn contains_alias(&self, alias: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let result: bool = obj.call_method1("contains_alias", (alias,))?.extract()?;
            Ok(result)
        })
    }

    /// Get candidates for an alias string.
    pub fn get_candidates(&self, alias: &str) -> Result<Vec<Candidate>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let candidates = obj.call_method1("get_alias_candidates", (alias,))?;
            let mut result = Vec::new();
            for candidate in candidates.iter()? {
                result.push(Candidate::from_py(candidate?)?);
            }
            Ok(result)
        })
    }

    /// Get the stored vector for an entity.
    pub fn get_vector(&self, entity: &str) -> Result<Vec<f32>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let vector = obj.call_method1("get_vector", (entity,))?;
            let mut result = Vec::new();
            for v in vector.iter()? {
                result.push(v?.extract()?);
            }
            Ok(result)
        })
    }

    /// Get prior probability for an alias→entity mapping.
    pub fn get_prior_prob(&self, alias: &str, entity: &str) -> Result<f32, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let prob: f32 = obj
                .call_method1("get_prior_prob", (alias, entity))?
                .extract()?;
            Ok(prob)
        })
    }

    /// Length of entity vectors.
    pub fn entity_vector_length(&self) -> Result<usize, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let len: usize = obj.getattr("entity_vector_length")?.extract()?;
            Ok(len)
        })
    }

    /// Whether the KB contains no entities or aliases.
    pub fn is_empty(&self) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let empty: bool = obj.call_method0("is_empty")?.extract()?;
            Ok(empty)
        })
    }

    /// Number of entities in the KB.
    pub fn get_size_entities(&self) -> Result<usize, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let size: usize = obj.call_method0("get_size_entities")?.extract()?;
            Ok(size)
        })
    }

    /// Number of aliases in the KB.
    pub fn get_size_aliases(&self) -> Result<usize, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let size: usize = obj.call_method0("get_size_aliases")?.extract()?;
            Ok(size)
        })
    }

    /// All entity strings.
    pub fn get_entity_strings(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let strings = obj.call_method0("get_entity_strings")?;
            let mut result = Vec::new();
            for s in strings.iter()? {
                result.push(s?.extract()?);
            }
            Ok(result)
        })
    }

    /// All alias strings.
    pub fn get_alias_strings(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let strings = obj.call_method0("get_alias_strings")?;
            let mut result = Vec::new();
            for s in strings.iter()? {
                result.push(s?.extract()?);
            }
            Ok(result)
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let bytes: Vec<u8> = obj.call_method0("to_bytes")?.extract()?;
            Ok(bytes)
        })
    }

    pub fn from_bytes(vocab: &Vocab, bytes: &[u8]) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.kb")?.getattr("InMemoryLookupKB")?;
            let kb = cls.call1((vocab.obj.bind(py), 0usize))?;
            kb.call_method1("from_bytes", (bytes,))?;
            Ok(KnowledgeBase { obj: kb.into() })
        })
    }

    pub fn to_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("to_disk", (path,))?;
            Ok(())
        })
    }

    pub fn from_disk(vocab: &Vocab, path: &str) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let cls = py.import_bound("spacy.kb")?.getattr("InMemoryLookupKB")?;
            let kb = cls.call1((vocab.obj.bind(py), 0usize))?;
            kb.call_method1("from_disk", (path,))?;
            Ok(KnowledgeBase { obj: kb.into() })
        })
    }
}

/// A candidate entity returned from a knowledge base lookup.
#[derive(Debug, Clone)]
pub struct Candidate {
    pub(crate) obj: Py<PyAny>,
}

impl Candidate {
    pub(crate) fn from_py(obj: Bound<'_, PyAny>) -> Result<Self, SpaCyError> {
        Ok(Candidate { obj: obj.into() })
    }

    pub fn entity_(&self) -> Result<String, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let s: String = obj.getattr("entity_")?.extract()?;
            Ok(s)
        })
    }

    pub fn alias_(&self) -> Result<String, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let s: String = obj.getattr("alias_")?.extract()?;
            Ok(s)
        })
    }

    pub fn prior_prob(&self) -> Result<f32, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let p: f32 = obj.getattr("prior_prob")?.extract()?;
            Ok(p)
        })
    }

    pub fn entity_vector(&self) -> Result<Vec<f32>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let vec = obj.getattr("entity_vector")?;
            let mut result = Vec::new();
            for v in vec.iter()? {
                result.push(v?.extract()?);
            }
            Ok(result)
        })
    }

    pub fn entity_freq(&self) -> Result<f32, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let f: f32 = obj.getattr("entity_freq")?.extract()?;
            Ok(f)
        })
    }
}
