use crate::utils::with_gil;
use crate::{Doc, SpaCyError, Vocab};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyDictMethods, PyList};

#[derive(Debug, Clone)]
pub struct Language {
    pub(crate) obj: Py<PyAny>,
}

pub struct DisabledPipesGuard<'a> {
    language: &'a Language,
    names: Vec<String>,
}

impl<'a> Drop for DisabledPipesGuard<'a> {
    fn drop(&mut self) {
        let _ = with_gil(|py| {
            let obj = self.language.obj.bind(py);
            for name in &self.names {
                let _ = obj.call_method1("enable_pipe", (name.as_str(),));
            }
            Ok(())
        });
    }
}

impl Language {
    pub fn load(model: &str) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let spacy = py.import_bound("spacy")?;
            let nlp = spacy.call_method1("load", (model,))?;
            Ok(Language { obj: nlp.into() })
        })
    }

    pub fn nlp(&self, text: &str) -> Result<Doc, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let doc = obj.call1((text,))?;
            Ok(Doc::new(doc.into()))
        })
    }

    pub fn pipe(&self, texts: &[&str]) -> Result<Vec<Doc>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let py_texts = PyList::new_bound(py, texts);
            let docs = obj.call_method1("pipe", (py_texts,))?;
            let mut result = Vec::new();
            for doc in docs.iter()? {
                let doc = doc?;
                result.push(Doc::new(doc.into()));
            }
            Ok(result)
        })
    }

    pub fn vocab(&self) -> Result<Vocab, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let vocab = obj.getattr("vocab")?;
            Ok(Vocab::new(vocab.into()))
        })
    }

    pub fn component_names(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let names: Vec<String> = obj.getattr("component_names")?.extract()?;
            Ok(names)
        })
    }

    pub fn add_pipe(&self, name: &str, config: Option<&str>, last: bool) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            if last {
                kwargs.set_item("last", true)?;
            }
            if let Some(cfg) = config {
                let cfg_dict: Bound<'_, PyDict> = py
                    .import_bound("json")?
                    .getattr("loads")?
                    .call1((cfg,))?
                    .extract()?;
                for (key, value) in &cfg_dict {
                    kwargs.set_item(key, value)?;
                }
            }
            obj.call_method("add_pipe", (name,), Some(&kwargs))?;
            Ok(())
        })
    }

    pub fn remove_pipe(&self, name: &str) -> Result<Py<PyAny>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let pipe = obj.call_method1("remove_pipe", (name,))?;
            Ok(pipe.into())
        })
    }

    pub fn get_pipe(&self, name: &str) -> Result<Py<PyAny>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let pipe = obj.getattr("get_pipe")?.call1((name,))?;
            Ok(pipe.into())
        })
    }

    pub fn disable_pipes(&self, names: &[&str]) -> Result<DisabledPipesGuard<'_>, SpaCyError> {
        let names_vec: Vec<String> = names.iter().map(|s| s.to_string()).collect();
        with_gil(|py| {
            let obj = self.obj.bind(py);
            for name in names {
                obj.call_method1("disable_pipe", (*name,))?;
            }
            Ok(())
        })?;
        Ok(DisabledPipesGuard {
            language: self,
            names: names_vec,
        })
    }

    pub fn to_disk(&self, path: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("to_disk", (path,))?;
            Ok(())
        })
    }

    pub fn replace_pipe(
        &self,
        name: &str,
        factory_name: &str,
        config: Option<&str>,
    ) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            if let Some(cfg) = config {
                let cfg_dict: Bound<'_, PyDict> = py
                    .import_bound("json")?
                    .getattr("loads")?
                    .call1((cfg,))?
                    .extract()?;
                for (key, value) in &cfg_dict {
                    kwargs.set_item(key, value)?;
                }
            }
            obj.call_method("replace_pipe", (name, factory_name), Some(&kwargs))?;
            Ok(())
        })
    }

    pub fn rename_pipe(&self, old_name: &str, new_name: &str) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("rename_pipe", (old_name, new_name))?;
            Ok(())
        })
    }

    pub fn pipe_names(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let names: Vec<String> = obj.getattr("pipe_names")?.extract()?;
            Ok(names)
        })
    }

    pub fn pipeline(&self) -> Result<Vec<(String, Py<PyAny>)>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let pipeline = obj.getattr("pipeline")?;
            let mut result = Vec::new();
            for item in pipeline.iter()? {
                let item = item?;
                let tuple = match item.downcast::<pyo3::types::PyTuple>() {
                    Ok(t) => t,
                    Err(e) => return Err(SpaCyError::Python(format!("Downcast error: {}", e))),
                };
                let name: String = tuple.get_item(0)?.extract()?;
                let component = tuple.get_item(1)?.into();
                result.push((name, component));
            }
            Ok(result)
        })
    }

    pub fn disabled(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let disabled = obj.getattr("disabled")?;
            let mut result = Vec::new();
            for name in disabled.iter()? {
                result.push(name?.extract()?);
            }
            Ok(result)
        })
    }

    pub fn from_disk(path: &str) -> Result<Self, SpaCyError> {
        with_gil(|py| {
            let spacy = py.import_bound("spacy")?;
            let nlp = spacy.call_method1("load", (path,))?;
            Ok(Language { obj: nlp.into() })
        })
    }
}
