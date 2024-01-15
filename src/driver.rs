use crate::track;
use pyo3::create_exception;
use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use songbird::{Config, Driver as SongbirdDriver};

create_exception!(dextbird, SetupError, pyo3::exceptions::PyException);
create_exception!(dextbird, JoinError, pyo3::exceptions::PyException);
create_exception!(dextbird, ConnectionError, pyo3::exceptions::PyException);

pub fn register_error(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("SetupError", py.get_type::<SetupError>())?;
    m.add("JoinError", py.get_type::<JoinError>())?;
    m.add("ConnectionError", py.get_type::<ConnectionError>())?;
    Ok(())
}

fn convert_error<T>(result: JoinResult<T>) -> Result<T, PyErr> {
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(JoinError::new_err(err.to_string())),
    }
}

#[pyclass]
pub struct Driver {
    driver: Arc<Mutex<SongbirdDriver>>,
}

#[pymethods]
impl Driver {
    #[new]
    fn new() -> Self {
        let driver = Arc::new(Mutex::new(SongbirdDriver::new(Config::default())));
        Self { driver }
    }
}