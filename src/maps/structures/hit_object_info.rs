use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct HitObjectInfo {
    pub start_time: i32,
    pub lane: i32,
    pub end_time: i32,
    // pub hit_sound: HitSounds,
}
