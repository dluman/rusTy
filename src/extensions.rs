use crate::utils::{pyobject_to_json, value_to_pyobject};
use crate::SpaCyError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Defines how a custom extension attribute is registered.
#[derive(Debug, Clone)]
pub enum ExtensionDefinition {
    /// Simple attribute with an optional default value.
    Attribute { default: serde_json::Value },
    /// Property with a getter (and optional setter) expressed as Python source.
    Property {
        getter: String,
        setter: Option<String>,
    },
    /// Method expressed as Python source.
    Method { method: String },
}

/// Information returned by `remove_extension`.
#[derive(Debug, Clone)]
pub struct ExtensionInfo {
    pub default: Option<serde_json::Value>,
    pub has_getter: bool,
    pub has_setter: bool,
    pub has_method: bool,
}

/// Internal helper to register an extension on a spaCy class.
pub(crate) fn py_set_extension(
    py: Python,
    cls_name: &str,
    name: &str,
    def: &ExtensionDefinition,
    force: bool,
) -> Result<(), SpaCyError> {
    let tokens_mod = py.import_bound("spacy.tokens")?;
    let cls = tokens_mod.getattr(cls_name)?;
    let kwargs = PyDict::new_bound(py);
    kwargs.set_item("force", force)?;
    match def {
        ExtensionDefinition::Attribute { default } => {
            kwargs.set_item("default", value_to_pyobject(py, default)?)?;
        }
        ExtensionDefinition::Property { getter, setter } => {
            let getter_py = py.eval_bound(getter, None, None)?;
            kwargs.set_item("getter", getter_py)?;
            if let Some(setter) = setter {
                let setter_py = py.eval_bound(setter, None, None)?;
                kwargs.set_item("setter", setter_py)?;
            }
        }
        ExtensionDefinition::Method { method } => {
            let method_py = py.eval_bound(method, None, None)?;
            kwargs.set_item("method", method_py)?;
        }
    }
    cls.call_method("set_extension", (name,), Some(&kwargs))?;
    Ok(())
}

/// Internal helper to check if an extension exists on a spaCy class.
pub(crate) fn py_has_extension(py: Python, cls_name: &str, name: &str) -> Result<bool, SpaCyError> {
    let tokens_mod = py.import_bound("spacy.tokens")?;
    let cls = tokens_mod.getattr(cls_name)?;
    let has: bool = cls.call_method1("has_extension", (name,))?.extract()?;
    Ok(has)
}

/// Internal helper to remove an extension from a spaCy class.
pub(crate) fn py_remove_extension(
    py: Python,
    cls_name: &str,
    name: &str,
) -> Result<ExtensionInfo, SpaCyError> {
    let tokens_mod = py.import_bound("spacy.tokens")?;
    let cls = tokens_mod.getattr(cls_name)?;
    let result = cls.call_method1("remove_extension", (name,))?;
    let default = result.get_item(0)?;
    let default = if default.is_none() {
        None
    } else {
        Some(pyobject_to_json(&default)?)
    };
    let has_getter: bool = !result.get_item(2)?.is_none();
    let has_setter: bool = !result.get_item(3)?.is_none();
    let has_method: bool = !result.get_item(1)?.is_none();
    Ok(ExtensionInfo {
        default,
        has_getter,
        has_setter,
        has_method,
    })
}

/// Internal helper to get a value from the `._` namespace.
pub(crate) fn py_get_underscore(
    obj: &Bound<'_, PyAny>,
    name: &str,
) -> Result<serde_json::Value, SpaCyError> {
    let underscore = obj.getattr("_")?;
    let val = underscore.getattr(name)?;
    pyobject_to_json(&val)
}

/// Internal helper to set a value in the `._` namespace.
pub(crate) fn py_set_underscore(
    obj: &Bound<'_, PyAny>,
    name: &str,
    value: serde_json::Value,
) -> Result<(), SpaCyError> {
    let py = obj.py();
    let underscore = obj.getattr("_")?;
    let py_value = value_to_pyobject(py, &value)?;
    underscore.setattr(name, py_value)?;
    Ok(())
}

/// Internal helper to check if a custom attribute exists on an instance.
pub(crate) fn py_has_underscore(obj: &Bound<'_, PyAny>, name: &str) -> Result<bool, SpaCyError> {
    let underscore = obj.getattr("_")?;
    let has: bool = underscore.call_method1("has", (name,))?.extract()?;
    Ok(has)
}

/// Internal helper to call a method extension from the `._` namespace.
pub(crate) fn py_call_underscore(
    obj: &Bound<'_, PyAny>,
    name: &str,
    args: &[serde_json::Value],
    kwargs: &std::collections::HashMap<String, serde_json::Value>,
) -> Result<serde_json::Value, SpaCyError> {
    let py = obj.py();
    let underscore = obj.getattr("_")?;
    let method = underscore.getattr(name)?;
    let py_args: Vec<PyObject> = args
        .iter()
        .map(|v| value_to_pyobject(py, v))
        .collect::<Result<Vec<_>, _>>()?;
    let py_args = pyo3::types::PyTuple::new_bound(py, py_args);
    let py_kwargs = crate::utils::values_to_pydict(py, kwargs)?;
    let result = method.call(&py_args, Some(&py_kwargs))?;
    pyobject_to_json(&result)
}
