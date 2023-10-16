use pyo3::prelude::*;

mod core;
mod track;
mod driver;
mod update_voice_state;

/// Core module
#[pymodule]
fn dextbird(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<crate::core::Core>()?;
    m.add_class::<driver::PipeDriver>()?;
    m.add_class::<track::Track>()?;
    crate::core::register_error(py, m)?;
    track::register_error(py, m)?;

    Ok(())
}
