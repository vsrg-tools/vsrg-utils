use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct SoundEffectInfo {
    pub start_time: f32,
    pub sample: i32,
    pub volume: i32,
}
