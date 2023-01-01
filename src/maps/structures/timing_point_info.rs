use crate::enums::quaver::TimeSignature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct TimingPointInfo {
    pub start_time: f32,
    pub bpm: f32,
    pub signature: TimeSignature,
    pub hidden: bool,
}
