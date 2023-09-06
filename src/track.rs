use songbird::tracks::TrackHandle;
use songbird::events::{EventHandler, EventContext, Event, TrackEvent};
use pyo3::prelude::*;
use pyo3::PyObject;
use async_trait::async_trait;
use std::sync::Arc;
use std::thread;

pub struct TrackAfterEvent {
    after_func: Arc<PyObject>,
}

#[async_trait]
impl EventHandler for TrackAfterEvent {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let after_func = Arc::clone(&self.after_func);
        thread::spawn(move || {
            Python::with_gil(|py| {
                after_func.call0(py).unwrap();
            });
        });
        return None;
    }
}

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

    pub fn after(&self, after_func: PyObject) -> PyResult<()> {
        let after_event = TrackAfterEvent {
            after_func: after_func.into(),
        };
        self.handle.add_event(Event::Track(TrackEvent::End), after_event).unwrap();
        Ok(())
    }
}