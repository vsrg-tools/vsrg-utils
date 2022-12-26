use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    EnumString, Display, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug,
)]
pub enum GameMode {
    #[default]
    Keys4 = 1,
    Keys7 = 2,
}

impl GameMode {
    pub fn key_count(self) -> u8 {
        match self {
            GameMode::Keys4 => 4,
            GameMode::Keys7 => 7,
        }
    }
}
