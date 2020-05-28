#[path = "dispatch.rs"] mod dispatch;
#[path = "types.rs"] mod types;

use cpython::*;

#[derive(Debug)]
pub struct Doc {
  pub object: Option<PyObject>,
  pub fields: serde_json::value::Value,
}

pub trait Callable<T> {
  fn call(self, method: &'static str) -> dispatch::Call<T>;
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

impl<T> Callable<T> for Doc {
  
  fn call(self, method: &'static str)
    -> dispatch::Call<T> {
      dispatch::Call::<T> {
        object: self.object,
        method: method,
        args: None,
        kwargs: "",
      }
    }
}

impl ToPyObject for Doc {
}

impl<T: ToPyObject> dispatch::Call<T> where T: Callable<PyObject> {
  
  pub fn args(mut self, args: T)
    -> Self {
      self.args = Some(args);
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
      let args = (self.args.unwrap()).to_py_object(python);
      let result = (self.object.unwrap())
        .call_method(
          python,
          self.method,
          (args,),
          None
        ).unwrap();
      println!("{}",self.method);
    }
}