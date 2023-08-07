use crate::update_voice_state::VoiceUpdate;
use pyo3::prelude::*;
use songbird::id::{ChannelId, GuildId, UserId};
use songbird::shards::Shard;
use songbird::Call;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyclass]
pub struct Core {
    call: Arc<Mutex<Call>>,
}

#[pymethods]
impl Core {
    #[new]
    pub fn new(py: Python<'_>, client: Py<PyAny>, guild_id: u64, user_id: u64) -> Self {
        let shard = Shard::Generic(Arc::new(VoiceUpdate {
            client: client.as_ref(py).clone().into(),
        }));
        let call = Call::new(GuildId(guild_id), shard, UserId(user_id));
        Self {
            call: Arc::new(Mutex::new(call)),
        }
    }

    pub fn join<'a>(&'a self, py: Python<'a>, channel_id: u64) -> PyResult<&PyAny> {
        let mut call = Arc::clone(&self.call);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut call = call.lock().await;
            call.join(ChannelId(channel_id)).await.unwrap();
            Ok(())
        })
    }
}
