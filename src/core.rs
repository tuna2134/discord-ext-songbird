use crate::update_voice_state::VoiceUpdate;
use songbird::shards::Shard;
use songbird::Call;
use songbird::id::{ChannelId, GuildId, UserId};
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
struct Core {
    call: Call,
}

#[pymethods]
impl Core {
    #[new]
    fn new(py: Python<'_>, client: Py<PyAny>, guild_id: u64, user_id: u64) -> Self {
        let shard = Shard::Generic(Arc::new(VoiceUpdate { client: client.as_ref(py).clone().into() } ));
        let call = Call::new(GuildId(guild_id), shard, UserId(user_id));
        Self { call }
    }
     
    fn join(&self, py: Python, channel_id: u64) -> PyResult<&PyAny> {
        let mut call = self.call.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            call.join(ChannelId(channel_id)).await.unwrap();
            Ok(())
        })
    }
}
