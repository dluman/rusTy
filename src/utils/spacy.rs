use crate::utils::doc;

use cpython::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Module {
    pub instance: Option<PyObject>,
}

// Initialise spacy model
lazy_static! {
  // spaCy instance
  static ref SPACY: Mutex<PyModule> = {
    // Acquire GIL
    let gil = Python::acquire_gil();
    let python = gil.python();
    // Import the module
    let spacy: PyModule = python
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
    pub fn init() -> Module {
        Module { instance: None }
    }

    // Load specific spacy model
    pub fn load(self, model: &'static str) -> Self {
        // Load module
        let spacy = SPACY.lock().unwrap();
        // Acquire GIL
        let gil = Python::acquire_gil();
        let python = gil.python();
        // Get the MODEL static
        let mut mdl = MODEL.lock().unwrap();
        mdl.instance = Some(spacy.call(python, "load", (model,), None).unwrap());
        self
    }
}

pub fn nlp(text: &'static str) -> doc::Doc {
    // Run nlp pipeline on text
    let gil = Python::acquire_gil();
    let python = gil.python();

    let model = MODEL.lock().unwrap();
    let result = (model.instance)
        .as_ref()
        .unwrap()
        .call(python, (text,), None)
        .unwrap();
    let doc = doc::Doc::new(python, result);
    doc
}

pub fn vocab() -> PyObject {
    // Run nlp pipeline on text
    let gil = Python::acquire_gil();
    let python = gil.python();

    let model = MODEL.lock().unwrap();
    let result = (model.instance)
        .as_ref()
        .unwrap()
        .getattr(python, "vocab")
        .unwrap();
    result
}

pub fn tokenizer(vocabulary: PyList) -> PyObject {
    // Run nlp pipeline on text
    let gil = Python::acquire_gil();
    let python = gil.python();

    let model = MODEL.lock().unwrap();
    let result = (model.instance)
        .as_ref()
        .unwrap()
        .getattr(python, "tokenizer")
        .unwrap()
        .getattr(python, "pipe")
        .unwrap()
        .call(python, (vocabulary,), None)
        .unwrap();
    result
}
