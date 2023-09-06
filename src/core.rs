use crate::update_voice_state::VoiceUpdate;
use pyo3::prelude::*;
use songbird::id::{ChannelId, GuildId, UserId};
use songbird::shards::Shard;
use songbird::ytdl;
use songbird::Call;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyfunction]
pub fn setup(py: Python<'_>, client: Py<PyAny>, guild_id: u64, user_id: u64) -> PyResult<&PyAny> {
    let shard = Shard::Generic(Arc::new(VoiceUpdate {
        client: client.as_ref(py).clone().into(),
    }));
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let call = Call::new(GuildId(guild_id), shard, UserId(user_id));
        Ok(Core {
            call: Arc::new(Mutex::new(call)),
        })
    })
}

#[pyclass]
pub struct Core {
    call: Arc<Mutex<Call>>,
}

#[pymethods]
impl Core {
    /*
    #[new]
    pub fn new(py: Python<'_>, client: Py<PyAny>, guild_id: u64, user_id: u64) -> Self {
        let rt = Builder::new_multi_thread();
        pyo3_asyncio::tokio::init(rt);
        let shard = Shard::Generic(Arc::new(VoiceUpdate {
            client: client.as_ref(py).clone().into(),
        }));
        let call = Call::new(GuildId(guild_id), shard, UserId(user_id));
        Self {
            call: Arc::new(Mutex::new(call)),
        }
    }
    */

    pub fn join<'a>(&'a self, py: Python<'a>, channel_id: u64) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            call.join(ChannelId(channel_id)).await.unwrap();
            Ok(())
        })
    }

    pub fn connect<'a>(&'a self, py: Python<'a>) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let call_unlock = call.lock().await;
            if let Some(connection_info) = call_unlock.current_connection() {
                println!("See connection info");
                let mut call_unlock = call.lock().await;
                call_unlock.connect(connection_info.clone()).await.unwrap();
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
            Ok(())
        })
    }

    pub fn ytdl<'a>(&'a self, py: Python<'a>, url: String) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            let input = ytdl(&url).await.unwrap();
            call.play_source(input).play().unwrap();
            Ok(())
        })
    }
}
