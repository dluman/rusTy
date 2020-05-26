#[path = "dispatch.rs"] mod dispatch;
#[path = "types.rs"] mod types;

use cpython::*; 

#[derive(Debug)]
pub struct Doc {
  pub object: Option<PyObject>,
  pub fields: serde_json::value::Value,
}

trait Callable {
  fn call(self, method: &'static str) -> dispatch::Call;
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

impl Callable for Doc {
  
  fn call(self, method: &'static str)
    -> dispatch::Call {
      dispatch::Call{
        object: self.object,
        method: method,
        args: "",
        kwargs: "",
      }
    }
}

impl dispatch::Call {
  
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
    
  pub fn invoke(self) 
    {//-> PyObject {
      // Acquire GIL
      let gil = Python::acquire_gil();
      let python = gil.python();
    }
}