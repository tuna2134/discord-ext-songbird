use crate::update_voice_state::VoiceUpdate;
use pyo3::prelude::*;
use songbird::id::{ChannelId, GuildId, UserId};
use songbird::shards::Shard;
use songbird::Call;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::runtime::Builder;

#[pyfunction]
pub fn setup(
    py: Python<'_>,
    client: Py<PyAny>,
    guild_id: u64,
    user_id: u64
) -> PyResult<&PyAny> {
    let shard = Shard::Generic(Arc::new(VoiceUpdate {
        client: client.as_ref(py).clone().into(),
    }));
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let call = Call::new(GuildId(guild_id), shard, UserId(user_id));
        Ok(Core { call: Arc::new(Mutex::new(call)) })
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

    pub fn update_server<'a>(&'a self, py: Python<'a>, endpoint: String, token: String) -> PyResult<&PyAny> {
        let call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            call.update_server(endpoint, token);
            Ok(())
        })
    }
}
