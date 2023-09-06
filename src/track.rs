use songbird::tracks::TrackHandle;
use pyo3::prelude::*;

use std::sync::Arc;

#[pyclass]
pub struct Track {
    pub handle: Arc<TrackHandle>,
}

#[pymethods]
impl Track {
    pub fn play(&self) -> PyResult<()> {
        self.handle.play().unwrap();
        Ok(())
    }

    pub fn enable_loop(&self) -> PyResult<()> {
        self.handle.enable_loop().unwrap();
        Ok(())
    }

    pub fn disable_loop(&self) -> PyResult<()> {
        self.handle.disable_loop().unwrap();
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) -> PyResult<()> {
        self.handle.set_volume(volume).unwrap();
        Ok(())
    }
}