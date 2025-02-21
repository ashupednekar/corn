use std::sync::Arc;
use pyo3::prelude::*;
pub mod request;
pub mod response;


pub struct WSGIApp {
    app: Arc<Py<PyAny>>,
}

impl WSGIApp {
    pub fn new(py: Python, module: &str, app: &str) -> PyResult<Self> {
        let module = py.import(module)?;
        let app = Arc::new(module.getattr(app)?.into_pyobject(py)?.into());
        Ok(Self { app })
    }
}
