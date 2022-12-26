use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CustomAudioSampleInfo {
    pub path: String,
    pub unaffected_by_rate: bool,
}
