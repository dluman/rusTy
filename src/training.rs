use crate::{utils::with_gil, Doc, SpaCyError};
use pyo3::prelude::*;

/// Convert entity offset annotations to BILUO tags for a `Doc`.
///
/// * `doc` — a `Doc` (usually from `nlp.make_doc(text)`)
/// * `entities` — list of `(start_char, end_char, label)` tuples
///
/// Returns a vector of BILUO tag strings (e.g., `"U-ORG"`, `"B-PER"`, `"I-PER"`, `"L-PER"`, `"O"`).
/// Misaligned tokens are marked `"-"`.
pub fn offsets_to_biluo_tags(
    doc: &Doc,
    entities: &[(usize, usize, &str)],
) -> Result<Vec<String>, SpaCyError> {
    with_gil(|py| {
        let func = py
            .import_bound("spacy.training")?
            .getattr("offsets_to_biluo_tags")?;
        let doc = doc.obj.bind(py);
        let entities_py = pyo3::types::PyList::empty_bound(py);
        for (start, end, label) in entities {
            let start_py = (*start).into_py(py);
            let end_py = (*end).into_py(py);
            let label_py = (*label).into_py(py);
            let tuple = pyo3::types::PyTuple::new_bound(py, [start_py, end_py, label_py]);
            entities_py.append(tuple)?;
        }
        let tags = func.call1((doc, entities_py))?;
        let mut result = Vec::new();
        for tag in tags.iter()? {
            result.push(tag?.extract()?);
        }
        Ok(result)
    })
}
