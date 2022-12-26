use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize, Default)]
    #[serde(rename_all = "PascalCase")]
    #[serde(default)]
    pub struct HitSounds: i32 {
        const NORMAL = 1 << 0;
        const WHISTLE = 1 << 1;
        const FINISH = 1 << 2;
        const CLAP = 1 << 3;
    }
}
