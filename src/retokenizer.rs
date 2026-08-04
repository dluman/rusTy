use crate::{utils::with_gil, SpaCyError, Span, Token};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

/// A dependency head for `RetokenizerGuard::split`.
/// Either a token in the original doc, or a (token, subtoken_index) pair.
#[derive(Debug, Clone)]
pub enum Head {
    Token(Token),
    Subtoken(Token, usize),
}

/// RAII guard around spaCy's `Doc.retokenize()` context manager.
/// On drop, `__exit__` is called and all pending merges/splits are applied.
pub struct RetokenizerGuard {
    retokenizer: Py<PyAny>,
}

impl RetokenizerGuard {
    pub(crate) fn new(retokenizer: Py<PyAny>) -> Self {
        RetokenizerGuard { retokenizer }
    }

    /// Merge the tokens in `span` into a single token.
    /// `attrs_json` is an optional JSON dict string of attributes to set.
    pub fn merge(&mut self, span: &Span, attrs_json: Option<&str>) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let retokenizer = self.retokenizer.bind(py);
            let span = span.obj.bind(py);
            let kwargs = PyDict::new_bound(py);
            if let Some(attrs) = attrs_json {
                let attrs_dict: Bound<'_, PyDict> = py
                    .import_bound("json")?
                    .getattr("loads")?
                    .call1((attrs,))?
                    .extract()?;
                kwargs.set_item("attrs", attrs_dict)?;
            }
            retokenizer.call_method("merge", (span,), Some(&kwargs))?;
            Ok(())
        })
    }

    /// Split `token` into subtokens with the given `orths`.
    /// `heads` specifies how each subtoken attaches to the dependency tree.
    /// `attrs_json` is an optional JSON dict string of per-token attributes.
    pub fn split(
        &mut self,
        token: &Token,
        orths: &[&str],
        heads: &[Head],
        attrs_json: Option<&str>,
    ) -> Result<(), SpaCyError> {
        with_gil(|py| {
            let retokenizer = self.retokenizer.bind(py);
            let token = token.obj.bind(py);

            let py_orths = PyList::new_bound(py, orths);

            let py_heads = PyList::empty_bound(py);
            for head in heads {
                match head {
                    Head::Token(t) => py_heads.append(t.obj.bind(py))?,
                    Head::Subtoken(t, idx) => {
                        let items = vec![t.obj.clone(), (*idx).into_py(py)];
                        let tuple = PyTuple::new_bound(py, items);
                        py_heads.append(tuple)?;
                    }
                }
            }

            let kwargs = PyDict::new_bound(py);
            if let Some(attrs) = attrs_json {
                let attrs_dict: Bound<'_, PyDict> = py
                    .import_bound("json")?
                    .getattr("loads")?
                    .call1((attrs,))?
                    .extract()?;
                kwargs.set_item("attrs", attrs_dict)?;
            }
            retokenizer.call_method("split", (token, py_orths, py_heads), Some(&kwargs))?;
            Ok(())
        })
    }
}

impl Drop for RetokenizerGuard {
    fn drop(&mut self) {
        let _ = with_gil(|py| {
            let retokenizer = self.retokenizer.bind(py);
            retokenizer.call_method1("__exit__", (py.None(), py.None(), py.None()))?;
            Ok(())
        });
    }
}
