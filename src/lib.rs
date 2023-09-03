use pyo3::prelude::*;

mod core;
mod update_voice_state;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn dextbird(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::Core>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(core::setup, m)?)?;
    Ok(())
}
