#[path = "doc.rs"] mod doc;

use cpython::*;
use lazy_static::lazy_static;
use std::default::Default;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use serde_json::*; 

#[derive(Debug)]
pub struct module {
  model: PyObject
}

pub struct nlpstruct {
  object: PyObject,
  container: PyType,
  pub fields: serde_json::value::Value,
  //pub methods: serde_json::value::Value,
}

#[derive(Debug)]
pub struct call {
  method: &'static str,
  args: &'static str,
  kwargs: &'static str,
}

pub trait callable {
  fn call(&self, func: &'static str) -> call;
  fn args(&self, args: &'static str);
  fn kwargs(&self, kwargs: &'static str);
  fn invoke(&self);
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

impl callable for nlpstruct {

  // Some quasi-builder monstrosity

  fn call(&self, func: &'static str)
    -> call {
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
      call{
        method: func,
        args: "",
        kwargs: "",
      }
    }
  
  fn args(&self, args: &'static str) {
    
  }
  
  fn kwargs(&self, kwargs: &'static str) {
  }
  
  fn invoke(&self) 
    {//-> PyObject {
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
      
      println!("{}",self.object.is_callable(python));
    }
}

impl call {
  
  pub fn args(mut self, args: &'static str)
    -> Self {
      self.args = args;
      self
    }
  
  pub fn kwargs(mut self, kwargs: &'static str) 
    -> Self {
      self.kwargs = kwargs;
      self
    }
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
    -> nlpstruct {
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
      self::module::map(
        (self.model)
          .call(python, (text,), None)
          .unwrap()
      )
  }
  
  fn map(object: PyObject) 
    -> nlpstruct {
    // Acquire GIL
    let gil = Python::acquire_gil();
    let python = gil.python();
    
    // Fields
    
    // Unwrap result
    let result = object
      .call_method(python,"to_json",("",),None)
      .unwrap();
    // Get the type
    let obj_type: PyType = object.get_type(python);
    // Convert PyString to a regular String
    let py_string = result
      .str(python)
      .unwrap();
    let mut string = String::from(
      py_string
        .to_string(python)
        .unwrap()
    );
    // IT'S A HACK!
    string = string.replace("'","\"");
    // Convert fields to a JSON structure
    let fields: Value = serde_json::from_str(&string)
      .unwrap();
    
    // Return the struct
    nlpstruct{
      object: object,
      container: obj_type, 
      fields: fields,
      //methods: methods.unwrap(),
    }
  }
}

// I use this to determine types and figure out what to do next.

pub fn eval(object: PyObject) {
    // Acquire GIL
    let gil = Python::acquire_gil();
    let python = gil.python();
    let obj_type: PyType = object.get_type(python);
    let obj_repr: PyResult<PyString> = object.repr(python);
    println!("{}",object.is_callable(python));
    println!("{:#?}",object.as_ptr());
    println!("{:#?}",obj_type.name(python));
    println!("{:#?}",obj_repr.unwrap().to_string(python).unwrap());
}