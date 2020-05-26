#[path = "doc.rs"] mod doc;

use cpython::*;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use serde_json::*; 
use std::sync::Mutex;

#[derive(Debug)]
pub struct Module {
  instance: Option<PyObject>,
}

// This requires all operations to be colocated?
lazy_static! {
  static ref SPACY: Mutex<PyModule> = {
    // Acquire GIL
    let gil = Python::acquire_gil();
    let python = gil.python();
    // Import the module
    let spacy = python
      .import("spacy")
      .unwrap();
    Mutex::new(spacy)
  };
}

impl Module {

  pub fn init()
    -> Module {
      Module {
        instance: None,
      }
    }
  
  pub fn load(mut self, model: &'static str)
    -> Self {
      // Load module
      let spacy = SPACY
        .lock()
        .unwrap();
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
      // Unwrap the result
      self.instance = Some(
        spacy
          .call(
            python,
            "load",
            (model,),
            None
          ).unwrap()
        );
      self
    }
    
  pub fn nlp(self, text: &'static str)
    -> doc::Doc {
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
      // Unwrap result
      let result = (self.instance)
        .unwrap()
        .call(
          python,
          (text,),
          None
        ).unwrap();
      doc::Doc::new(result)
    }
}