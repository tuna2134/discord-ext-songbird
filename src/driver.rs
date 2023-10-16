use pyo3::prelude::*;
use pyo3::create_exception;
use songbird::{
    Driver, Config
};

create_exception!(driver, DriverError, pyo3::exceptions::PyException);

#[pyclasses]
struct PipeDriver {
    driver: Driver,
}

#[pymethods]
impl PipeDriver {
    #[new]
    fn new() -> Self {
        Self {
            driver: Driver::new(Config::default()),
        }
    }
}