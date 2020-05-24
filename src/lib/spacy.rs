#[path = "doc.rs"] mod doc;

use cpython::*;
use lazy_static::lazy_static;
use std::default::Default;
use std::borrow::Borrow;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use serde_json::*; 

#[derive(Debug)]
pub struct module {
  model: PyObject
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

impl module {

  pub fn load(model: &'static str)
    -> module {
      // Reference module
      let spacy = SPACY
        .lock()
        .unwrap();
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
      // Unwrap the result
      module {
        model: spacy
          .call(python, "load", (model,), None)
          .unwrap(),
      }
    }
    
  pub fn nlp(self, text: &'static str) 
    -> PyObject {
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
      // Unwrap result
      (self.model)
        .call(python, (text,), None)
        .unwrap()
  }
  
  pub fn map(object: PyObject) {
    // Acquire GIL
    let gil = Python::acquire_gil();
    let python = gil.python();
    let result = object
      .call_method(python,"to_json",("",),None)
      .unwrap();
    let py_string = result
      .str(python)
      .unwrap();
    let mut string = String::from(
      py_string
        .to_string(python)
        .unwrap()
    );
    string = string.replace("'","\"");
    let json: Value = serde_json::from_str(&string)
      .unwrap();
    println!("{:?}",json);
  }
}

/*pub fn eval(object: PyObject) {
    // Acquire GIL
    let gil = Python::acquire_gil();
    let python = gil.python();
    let obj_type: PyType = object.get_type(python);
    let obj_repr: PyResult<PyString> = object.repr(python);
    println!("{}",object.is_callable(python));
    println!("{:#?}",object.as_ptr());
    println!("{:#?}",obj_type.name(python));
    println!("{:#?}",obj_repr.unwrap().to_string(python).unwrap());
}*/