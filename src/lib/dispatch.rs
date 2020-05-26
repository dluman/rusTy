use cpython::*; 

pub struct Call {
  pub object: Option<PyObject>,
  pub method: &'static str,
  pub args: &'static str,
  pub kwargs: &'static str,
}