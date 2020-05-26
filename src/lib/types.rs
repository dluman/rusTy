use cpython::*;

pub fn map(object: PyObject) 
  -> serde_json::value::Value {
    // Acquire GIL
    let gil = Python::acquire_gil();
    let python = gil.python();
    
    // Fields

    // Unwrap result
    let result = object
      .call_method(python,"to_json",("",),None)
      .unwrap();
      
    // Get the type
    //let obj_type: PyType = object
      //.get_type(python);
    
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
    serde_json::from_str(&string)
      .unwrap()
  }