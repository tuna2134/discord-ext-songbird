use pyo3::prelude::*;

mod driver;
mod track;
mod update_voice_state;

/// Core module
#[pymodule]
fn dextbird(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<crate::driver::Driver>()?;
    m.add_class::<track::Track>()?;
    crate::driver::register_error(py, m)?;
    track::register_error(py, m)?;

    Ok(())
}
