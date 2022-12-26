use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct SliderVelocityInfo {
    pub start_time: f32,
    pub multiplier: f32,
}
