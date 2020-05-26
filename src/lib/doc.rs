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

  pub fn new(object: PyObject)
    -> Self {
      // Need to reconcile move
      let fields = types::map(object);
      Doc {
        object: None,
        fields: fields,
      }
    }
}