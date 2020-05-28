#[path = "doc.rs"] pub mod doc;

use cpython::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Module {
  pub instance: Option<PyObject>,
}

// This requires all operations to be colocated?
lazy_static! {
  // spaCy instance
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
  
  // spaCy instance with model loaded
  pub static ref MODEL: Mutex<Module> = {
    Mutex::new(
      Module::init()
    )
  };
}

impl Module {

  pub fn init()
    -> Module {
      Module {
        instance: None,
      }
    }
  
  pub fn load(self, model: &'static str)
    -> Self {
      // Load module
      let spacy = SPACY
        .lock()
        .unwrap();
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
      // Get the MODEL static
      let mut mdl = MODEL
        .lock()
        .unwrap();
      mdl.instance = Some(
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
}

// FREE RANGE FUNCTIONS

pub fn nlp(text: &'static str)
  -> doc::Doc {
    // Acquire GIL
    let gil = Python::acquire_gil();
    let python = gil.python();
    // Get the MODEL static
    let model = MODEL
      .lock()
      .unwrap();
    let result = (model.instance)
      .as_ref()
      .unwrap()
      .call(
        python,
        (text,),
        None
      ).unwrap();
    let doc = doc::Doc::new(python, result);
    doc
  }