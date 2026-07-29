use crate::{utils::with_gil, SpaCyError, Span};
use pyo3::prelude::*;
use pyo3::types::PyList;

/// A proxy to `Doc.spans` — a dict of named span groups.
#[derive(Debug, Clone)]
pub struct SpanGroups {
    pub(crate) obj: Py<PyAny>,
    pub(crate) doc: Py<PyAny>,
}

/// A named group of potentially overlapping spans.
#[derive(Debug, Clone)]
pub struct SpanGroup {
    pub(crate) obj: Py<PyAny>,
    pub(crate) doc: Py<PyAny>,
}

impl SpanGroups {
    pub fn get(&self, name: &str) -> Result<Option<SpanGroup>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let group = obj.get_item(name)?;
            if group.is_none() {
                Ok(None)
            } else {
                Ok(Some(SpanGroup {
                    obj: group.into(),
                    doc: self.doc.clone(),
                }))
            }
        })
    }

    pub fn set(&self, name: &str, spans: &[Span]) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let py_spans = PyList::empty_bound(py);
            for span in spans {
                py_spans.append(span.obj.bind(py))?;
            }
            obj.set_item(name, py_spans)?;
            Ok(())
        })
    }

    pub fn names(&self) -> Result<Vec<String>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let keys = obj.call_method0("keys")?;
            let mut result = Vec::new();
            for key in keys.iter()? {
                result.push(key?.extract()?);
            }
            Ok(result)
        })
    }

    pub fn has(&self, name: &str) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let contains: bool = obj.call_method1("__contains__", (name,))?.extract()?;
            Ok(contains)
        })
    }
}

impl SpanGroup {
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

    pub fn spans(&self) -> Result<Vec<Span>, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let mut result = Vec::new();
            for span in obj.iter()? {
                result.push(Span::new(span?.into()));
            }
            Ok(result)
        })
    }

    pub fn get(&self, index: usize) -> Result<Span, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let span = obj.call_method1("__getitem__", (index,))?;
            Ok(Span::new(span.into()))
        })
    }

    pub fn set(&self, index: usize, span: &Span) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("__setitem__", (index, span.obj.bind(py)))?;
            Ok(())
        })
    }

    pub fn remove(&self, index: usize) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("__delitem__", (index,))?;
            Ok(())
        })
    }

    pub fn append(&self, span: &Span) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            obj.call_method1("append", (span.obj.bind(py),))?;
            Ok(())
        })
    }

    pub fn extend(&self, spans: &[Span]) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let py_spans = PyList::empty_bound(py);
            for span in spans {
                py_spans.append(span.obj.bind(py))?;
            }
            obj.call_method1("extend", (py_spans,))?;
            Ok(())
        })
    }

    pub fn has_overlap(&self) -> Result<bool, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let val: bool = obj.getattr("has_overlap")?.extract()?;
            Ok(val)
        })
    }

    pub fn copy(&self) -> Result<SpanGroup, SpaCyError> {
        with_gil(|py| {
            let obj = self.obj.bind(py);
            let copy = obj.call_method0("copy")?;
            Ok(SpanGroup {
                obj: copy.into(),
                doc: self.doc.clone(),
            })
        })
    }
}
