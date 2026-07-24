use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpaCyError {
    #[error("Python error: {0}")]
    Python(String),

    #[error("Model not loaded")]
    ModelNotLoaded,

    #[error("Invalid index: {0}")]
    InvalidIndex(usize),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Numpy error: {0}")]
    Numpy(String),
}

impl From<pyo3::PyErr> for SpaCyError {
    fn from(err: pyo3::PyErr) -> Self {
        SpaCyError::Python(err.to_string())
    }
}

impl From<numpy::FromVecError> for SpaCyError {
    fn from(err: numpy::FromVecError) -> Self {
        SpaCyError::Numpy(err.to_string())
    }
}
