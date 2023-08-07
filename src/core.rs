use crate::update_voice_state::UpdateVoice;
use songbird::shards::Shard;
use songbird::Call;

#[pyclass]
struct Core {
    call: Call,
}

#[pymethods]
impl Core {
}
