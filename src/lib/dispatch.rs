use cpython::*; 

#[derive(Debug)]
pub struct Call<T>{
  pub object: Option<PyObject>,
  pub method: &'static str,
  pub args: Option<T>,
  pub kwargs: &'static str,
}