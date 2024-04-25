use crate::track;
use crate::update_voice_state::VoiceUpdate;
use pyo3::create_exception;
use pyo3::prelude::*;
use reqwest::Client;
use songbird::error::JoinResult;
use songbird::id::{ChannelId, GuildId, UserId};

use async_dropper::AsyncDrop;
use songbird::input::YoutubeDl;
use songbird::shards::Shard;
use songbird::Call;
use std::num::NonZeroU64;
use std::sync::Arc;
use tokio::sync::Mutex;

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
pub struct Core {
    call: Arc<Mutex<Call>>,
}

#[pymethods]
impl Core {
    #[new]
    pub fn new() -> PyResult<Self> {
        Err(SetupError::new_err("Use create function"))
    }

    // Setup core and songbird
    #[staticmethod]
    pub async fn setup<'a>(client: Py<PyAny>, guild_id: u64, user_id: u64) -> PyResult<Self> {
        let shard = Shard::Generic(Arc::new(VoiceUpdate {
            client: client.as_ref(py).into(),
        }));
        let call = Call::new(
            GuildId(NonZeroU64::new(guild_id).unwrap()),
            shard,
             UserId(NonZeroU64::new(user_id).unwrap()),
        );
        log::info!("Setup end");
        Ok(Self {
            call: Arc::new(Mutex::new(call)),
        })
    }

    pub async fn join<'a>(&'a self, channel_id: u64) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.lock().await;
        convert_error(
            call.join(ChannelId(NonZeroU64::new(channel_id).unwrap()))
                .await,
        )?;
        Ok(())
    }

    pub async fn connect<'a>(&'a self) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.lock().await;
        if call.current_connection().is_some() {
            let info = call.current_connection().unwrap().clone();
            let result = match call.connect(info).await {
                Ok(_) => {
                    log::info!("Connected");
                    Ok(())
                }
                Err(err) => Err(ConnectionError::new_err(err.to_string())),
            };
            result?
        }
        Ok(())
    }

    pub async fn update_server<'a>(
        &'a self,
        endpoint: String,
        token: String,
    ) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.lock().await;
        call.update_server(endpoint, token);
        log::info!("Update server");
        Ok(())
    }

    pub async fn update_state<'a>(
        &'a self,
        session_id: String,
        channel_id: Option<String>,
    ) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let channelid = if let Some(chid) = channel_id {
            Some(ChannelId(
                NonZeroU64::new(chid.parse::<u64>().unwrap()).unwrap(),
            ))
        } else {
            None
        };
        let mut call = call.lock().await;
        call.update_state(session_id, channelid);
        log::info!("Update voice state");
        Ok(())
    }

    pub async fn ytdl<'a>(&'a self, url: String) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        log::info!("Starting player");
        log::info!("Waiting call lock");
        let mut call = call.lock().await;
        log::info!("Create yotuube");
        let input = YoutubeDl::new(Client::new(), url);
        let track = call.play_input(input.clone().into());
        log::info!("Play it");
        Ok(track::Track::from_handle(track.into()))
    }

    pub fn source(&self, data: Vec<u8>) -> PyResult<track::Track> {
        let call = Arc::clone(&self.call);
        let mut call = call.blocking_lock();
        /*
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
        */
        Ok(track::Track::from_handle(
            call.play_input(data.into()).into(),
        ))
    }

    pub async fn deafen<'a>(&'a self, deaf: bool) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.lock().await;
        convert_error(call.deafen(deaf).await)?;
        log::info!("Connection is now deaf");
        Ok(())
    }

    pub async fn mute<'a>(&'a self, mute: bool) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.lock().await;
        convert_error(call.mute(mute).await)?;
        log::info!("Connection is now mute");
        Ok(())
    }

    pub async fn leave<'a>(&'a self) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.lock().await;
        convert_error(call.leave().await)?;
        log::info!("Leave from vc");
        Ok(())
    }

    pub fn stop(&self) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.blocking_lock();
        call.stop();
        log::info!("Stop to play voice");
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncDrop for Core {
    async fn async_drop<'a>(&'a mut self) {
        //let rt = pyo3_asyncio::tokio::get_runtime();
        //let call = Arc::clone(&self.call);
        //let _leave = rt.spawn_blocking(async move {
        log::info!("Dropping...");
        let mut call = self.call.lock().await;
        if call.leave().await.is_ok() {
            log::info!("Leave from something")
        }
        //});
        println!("Drop it");
    }
}
