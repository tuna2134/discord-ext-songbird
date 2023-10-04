use crate::track;
// use crate::update_voice_state::VoiceUpdate;
use pyo3::create_exception;
use pyo3::prelude::*;
use songbird::error::JoinResult;
use songbird::id::{ChannelId, GuildId, UserId};
use songbird::input;
use songbird::shards::Shard;
use songbird::ytdl;
use songbird::Config;
use songbird::Driver;
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

struct ConnectionInfo {
    pub channel_id: Option<ChannelId>,
    pub endpoint: Option<String>,
    pub guild_id: GuildId,
    pub session_id: Option<String>,
    pub token: Option<String>,
    pub user_id: UserId,
}

#[pyclass]
pub struct Core {
    driver: Arc<Mutex<Driver>>,
    conn_info: Arc<Mutex<ConnectionInfo>>,
}

#[pymethods]
impl Core {
    #[new]
    pub fn new() -> PyResult<Self> {
        Err(SetupError::new_err("Use create function"))
    }

    // Setup core and songbird
    #[staticmethod]
    pub fn setup(py: Python, user_id: u64, guild_id: u64) -> PyResult<&PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let driver = Driver::new(Config::default());
            log::info!("Setup end");
            Ok(Self {
                driver: Arc::new(Mutex::new(driver)),
                conn_info: Arc::new(Mutex::new(ConnectionInfo {
                    channel_id: None,
                    endpoint: None,
                    guild_id: GuildId(guild_id),
                    user_id: UserId(user_id),
                    session_id: None,
                    token: None,
                })),
            })
        })
    }

    /*
    pub fn join<'a>(&'a self, py: Python<'a>, channel_id: u64) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        let conn_info = Arc
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            convert_error(call.join(ChannelId(channel_id)).await)?;
    */

    pub fn connect<'a>(&'a self, py: Python<'a>) -> PyResult<&PyAny> {
        let driver = Arc::clone(&self.driver);
        let conn_info = Arc::clone(&self.conn_info);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let conn_info = conn_info.lock().await;
            let conn_info = songbird::ConnectionInfo {
                channel_id: conn_info.channel_id,
                endpoint: conn_info.endpoint.clone().unwrap(),
                guild_id: conn_info.guild_id,
                session_id: conn_info.session_id.clone().unwrap(),
                token: conn_info.token.clone().unwrap(),
                user_id: conn_info.user_id,
            };
            let mut driver = driver.lock().await;
            let result = match driver.connect(conn_info).await {
                Ok(_) => {
                    log::info!("Connected");
                    Ok(())
                }
                Err(err) => Err(ConnectionError::new_err(err.to_string())),
            };
            result
        })
    }

    pub fn update_server(&self, endpoint: String, token: String) -> PyResult<()> {
        let mut conn_info = self.conn_info.blocking_lock();
        conn_info.token = Some(token);
        conn_info.endpoint = Some(endpoint);
        Ok(())
    }

    pub fn update_state(
        &self,
        py: Python,
        session_id: String,
        channel_id: Option<String>,
    ) -> PyResult<()> {
        let call = Arc::clone(&self.driver);
        let mut channelid = None;
        if let Some(chid) = channel_id {
            channelid = Some(ChannelId(chid.parse::<u64>().unwrap()))
        }
        let mut conn_info = self.conn_info.blocking_lock();
        conn_info.channel_id = channelid;
        conn_info.session_id = Some(session_id);
        /*
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            call.update_state(session_id, channelid);
            log::info!("Update voice state");
            Ok(())
        })
        */
        Ok(())
    }

    
    pub fn ytdl<'a>(&'a self, py: Python<'a>, url: String) -> PyResult<&PyAny> {
        let driver = Arc::clone(&self.driver);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut driver = driver.lock().await;
            let input = ytdl(&url).await.unwrap();
            let track = driver.play_source(input);
            Ok(track::Track::from_handle(track.into()))
        })
    }

    /*
    pub fn source(&self, data: Vec<u8>, opus: bool) -> PyResult<track::Track> {
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
        Ok(track::Track::from_handle(
            call.play_source(input_source).into(),
        ))
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
    }*/

    pub fn leave<'a>(&'a self, py: Python<'a>) -> PyResult<&PyAny> {
        let driver = Arc::clone(&self.driver);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut driver = driver.lock().await;
            driver.leave();
            log::info!("Leave from vc");
            Ok(())
        })
    }

    /*pub fn stop(&self) -> PyResult<()> {
        let call = Arc::clone(&self.call);
        let mut call = call.blocking_lock();
        call.stop();
        log::info!("Stop to play voice");
        Ok(())
    }
    */
}
