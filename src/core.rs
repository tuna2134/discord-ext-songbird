use crate::update_voice_state::VoiceUpdate;
use crate::track;
use pyo3::prelude::*;
use songbird::id::{ChannelId, GuildId, UserId};
use songbird::input;
use songbird::shards::Shard;
use songbird::ytdl;
use songbird::Call;
use songbird::error::JoinResult;
use std::sync::Arc;
use tokio::sync::Mutex;
use pyo3::create_exception;

create_exception!(dextbird, SetupError, pyo3::exceptions::PyException);
create_exception!(dextbird, JoinError, pyo3::exceptions::PyException);

// Setup VoiceClient Core
#[pyfunction]
pub fn setup(py: Python<'_>, client: Py<PyAny>, guild_id: u64, user_id: u64) -> PyResult<&PyAny> {
    let shard = Shard::Generic(Arc::new(VoiceUpdate {
        client: client.as_ref(py).clone().into(),
    }));
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let call = Call::new(GuildId(guild_id), shard, UserId(user_id));
        log::info!("Setup end");
        Ok(Core {
            call: Arc::new(Mutex::new(call)),
        })
    })
}

fn convert_error<T>(result: JoinResult<T>) -> Result<T, PyErr> {
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(JoinError::new_err(err.to_string()))
    }
}

#[pyclass]
pub struct Core {
    call: Arc<Mutex<Call>>,
}

#[pymethods]
impl Core {
    #[new]
    pub fn new() -> PyResult<Self> {
        Err(SetupError::new_err("Use create function"))
    }

    #[staticmethod]
    pub fn setup<'a>(py: Python<'a>, client: Py<PyAny>, guild_id: u64, user_id: u64) -> PyResult<&PyAny> {
        let shard = Shard::Generic(Arc::new(VoiceUpdate {
            client: client.as_ref(py).clone().into(),
        }));
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let call = Call::new(GuildId(guild_id), shard, UserId(user_id));
            log::info!("Setup end");
            Ok(Self {
                call: Arc::new(Mutex::new(call)),
            })
        })
    }

    pub fn join<'a>(&'a self, py: Python<'a>, channel_id: u64) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            convert_error(call.join(ChannelId(channel_id)).await)?;
            Ok(())
        })
    }

    pub fn connect<'a>(&'a self, py: Python<'a>) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            if call.current_connection().is_some() {
                let info = call.current_connection().unwrap().clone();
                call.connect(info).await.unwrap();
                log::info!("Connected");
            }
            Ok(())
        })
    }

    pub fn update_server<'a>(
        &'a self,
        py: Python<'a>,
        endpoint: String,
        token: String,
    ) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            call.update_server(endpoint, token);
            log::info!("Update server");
            Ok(())
        })
    }

    pub fn update_state<'a>(
        &'a self,
        py: Python<'a>,
        session_id: String,
        channel_id: Option<String>,
    ) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        let mut channelid = None;
        if let Some(chid) = channel_id {
            channelid = Some(ChannelId(chid.parse::<u64>().unwrap()))
        }
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            call.update_state(session_id, channelid);
            log::info!("Update voice state");
            Ok(())
        })
    }

    pub fn ytdl<'a>(&'a self, py: Python<'a>, url: String) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            let input = ytdl(&url).await.unwrap();
            let track = call.play_source(input);
            Ok(track::Track::from_handle(track.into()))
        })
    }

    pub fn source<'a>(&'a self, data: Vec<u8>, opus: bool) -> PyResult<track::Track> {
        let call = Arc::clone(&self.call);
        let mut call = call.blocking_lock();
        let mut codec = input::Codec::Pcm;
        if opus {
            codec = input::Codec::Opus(input::codec::OpusDecoderState::new().unwrap());
        };
        let input_source = input::Input::new(
            true,
            input::Reader::from_memory(data),
            codec,
            input::Container::Raw,
            None,
        );
        Ok(track::Track::from_handle(call.play_source(input_source).into()))
    }

    pub fn deafen<'a>(&'a self, py: Python<'a>, deaf: bool) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            convert_error(call.deafen(deaf).await)?;
            log::info!("Connection is now deaf");
            Ok(())
        })
    }

    pub fn mute<'a>(&'a self, py: Python<'a>, mute: bool) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            convert_error(call.mute(mute).await)?;
            log::info!("Connection is now mute");
            Ok(())
        })
    }

    pub fn leave<'a>(&'a self, py: Python<'a>) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            convert_error(call.leave().await)?;
            log::info!("Leave from vc");
            Ok(())
        })
    }

    pub fn stop<'a>(&'a self) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.blocking_lock();
        call.stop();
        log::info!("Stop to play voice");
        Ok(())
    }
}
