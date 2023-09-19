use pyo3::prelude::*;

mod core;
mod track;
mod update_voice_state;

/// Core module
#[pymodule]
fn dextbird(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<crate::core::Core>()?;
    m.add_class::<track::Track>()?;
    m.add_function(wrap_pyfunction!(crate::core::setup, m)?)?;
    Ok(())
}
