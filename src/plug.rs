use pyo3::prelude::*;
use pyo3::PyObject;

async fn change_voice_state(py: Python, cli: PyObject, guild_id: u64, channel_id: Option<u64>, self_deaf: bool, self_mute: bool) {
};
