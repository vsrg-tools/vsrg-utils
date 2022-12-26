use serde::{de::Error, Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Debug)]
pub enum TimeSignature {
    Quadruple = 4,
    Triple = 3,
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self::Quadruple
    }
}

impl<'de> Deserialize<'de> for TimeSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buffer: &str = Deserialize::deserialize(deserializer)?;
        let parsed: u8 = buffer.parse().map_err(D::Error::custom)?;
        Ok(match parsed {
            4 => TimeSignature::Quadruple,
            3 => TimeSignature::Triple,
            _ => unreachable!(),
        })
    }
}
