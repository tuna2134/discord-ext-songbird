use pyo3::prelude::*;
use pyo3::types::PyDict;
use songbird::error::JoinResult;

async fn update_voice_state(
    client: Py<PyAny>,
    guild_id: u64,
    channel_id: Option<u64>,
    self_deaf: bool,
    self_mute: bool,
) -> PyResult<()> {
    let result = Python::with_gil(|py| {
        let plug = py.import("dextbird.plug")?;
        let option = PyDict::new(py);
        option.set_item("guild_id", guild_id)?;
        option.set_item("channel_id", channel_id)?;
        option.set_item("self_deaf", self_deaf)?;
        option.set_item("self_mute", self_mute)?;
        let uvc_func = pyo3_asyncio::tokio::into_future(
            plug.call_method1("update_voice_state", (client, option))?,
        );
        uvc_func
    });
    result.unwrap().await?;
    Ok(())
}

pub struct VoiceUpdate {
    pub client: Py<PyAny>,
}

#[async_trait::async_trait]
impl songbird::shards::VoiceUpdate for VoiceUpdate {
    async fn update_voice_state(
        &self,
        guild_id: songbird::id::GuildId,
        channel_id: Option<songbird::id::ChannelId>,
        self_deaf: bool,
        self_mute: bool,
    ) -> JoinResult<()> {
        let client = self.client.clone();
        let mut ch_id = None;
        if let Some(cid) = channel_id {
            ch_id = Some(cid.0);
        }
        let uvc_func = update_voice_state(client, guild_id.0, ch_id, self_deaf, self_mute);
        uvc_func.await.unwrap();
        Ok(())
    }
}
