#[path = "types.rs"] mod types;

use cpython::*;
use serde::{Serialize, Deserialize};
use serde_json::*; 

#[derive(Debug)]
pub struct Doc {
  pub object: Option<PyObject>,
  pub fields: serde_json::value::Value,
}

impl Doc {

  pub fn new(python: Python, object: PyObject)
    -> Self {
      let obj = object.clone_ref(python);
      let fields = types::map(obj);
      Doc {
        object: Some(object),
        fields: fields,
      }
    }
}