use songbird::tracks::TrackHandle;
use songbird::events::{EventHandler, EventContext, Event, TrackEvent};
use songbird::error::TrackResult;
use pyo3::prelude::*;
use pyo3::PyObject;
use pyo3::create_exception;
use async_trait::async_trait;
use std::sync::Arc;
use std::thread;

create_exception!(dextbird, TrackError, pyo3::exceptions::PyException);

pub struct TrackAfterEvent {
    after_func: Arc<PyObject>,
}

#[async_trait]
impl EventHandler for TrackAfterEvent {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let after_func = Arc::clone(&self.after_func);
        log::debug!("After function");
        thread::spawn(move || {
            Python::with_gil(|py| {
                after_func.call0(py).unwrap();
            });
        });
        return None;
    }
}

fn convert_error<T>(result: TrackResult<T>) -> PyResult<T> {
    match result {
        Ok(r) => Ok(r),
        Err(err) => Err(TrackError::new_err(err.to_string()))
    }
}

#[pyclass]
pub struct Track {
    pub handle: Arc<TrackHandle>,
}

impl Track {
    pub fn from_handle(track_handle: Arc<TrackHandle>) -> Self {
        Self {
            handle: track_handle,
        }
    }
}

#[pymethods]
impl Track {
    pub fn play(&self) -> PyResult<()> {
        convert_error(self.handle.play())?;
        Ok(())
    }

    pub fn enable_loop(&self) -> PyResult<()> {
        convert_error(self.handle.enable_loop())?;
        Ok(())
    }

    pub fn disable_loop(&self) -> PyResult<()> {
        convert_error(self.handle.disable_loop())?;
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) -> PyResult<()> {
        convert_error(self.handle.set_volume(volume))?;
        Ok(())
    }

    pub fn after(&self, after_func: PyObject) -> PyResult<()> {
        let after_event = TrackAfterEvent {
            after_func: after_func.into(),
        };
        convert_error(self.handle.add_event(Event::Track(TrackEvent::End), after_event))?;
        Ok(())
    }

    pub fn pause(&self) -> PyResult<()> {
        convert_error(self.handle.pause())?;
        Ok(())
    }

    pub fn stop(&self) -> PyResult<()> {
        convert_error(self.handle.stop())?;
        Ok(())
    }
}
