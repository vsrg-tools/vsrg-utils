use crate::enums::osu::ModIdentifier;
use crate::helpers::BinaryHelper;
use anyhow::{anyhow, Result};
use lzma_rs::lzma_decompress;
use std::fs;

use super::KeyPressState;

#[derive(Default)]
pub struct OsuReplay {
    pub mode: u8,
    pub version: u32,
    pub map_md5: String,
    pub username: String,
    pub replay_md5: String,
    pub count_300: u16,
    pub count_100: u16,
    pub count_50: u16,
    pub count_geki: u16,
    pub count_katu: u16,
    pub count_miss: u16,
    pub score: u32,
    pub max_combo: u16,
    pub perfect: u8,
    pub mods: ModIdentifier,
    pub life_bar: String,
    pub time_stamp: u64,
    pub frames: Vec<ReplayFrame>,
    pub rng_seed: u32,
}

impl OsuReplay {
    pub fn from_path(path: &str) -> Result<Self> {
        let mut self_ = Self {
            ..Default::default()
        };

        let data = fs::read(path)?;

        self_.read_replay(data)?;

        Ok(self_)
    }

    fn read_replay(&mut self, data: Vec<u8>) -> Result<()> {
        let mut br = BinaryHelper::from_u8(&data);

        self.mode = br.read_u8()?;

        if self.mode != 3 {
            return Err(anyhow!("Not a mania replay!"));
        }

        self.version = br.read_u32()?;
        self.map_md5 = br.read_osu_string()?;
        self.username = br.read_osu_string()?;
        self.replay_md5 = br.read_osu_string()?;
        self.count_300 = br.read_u16()?;
        self.count_100 = br.read_u16()?;
        self.count_50 = br.read_u16()?;
        self.count_geki = br.read_u16()?;
        self.count_katu = br.read_u16()?;
        self.count_miss = br.read_u16()?;
        self.score = br.read_u32()?;
        self.max_combo = br.read_u16()?;
        self.perfect = br.read_u8()?;

        let mod_bits = br.read_u32()?;
        self.mods = ModIdentifier::from_bits(mod_bits).unwrap();

        self.life_bar = br.read_osu_string()?;
        self.time_stamp = br.read_u64()?;

        let length = br.read_u32()?;
        let mut remaining_bytes = br.read_bytes(length as usize).unwrap();
        let mut decomp: Vec<u8> = Vec::new();
        lzma_decompress(&mut remaining_bytes, &mut decomp).unwrap();

        let frames: Vec<&str> = std::str::from_utf8(&decomp)
            .unwrap()
            .split(",")
            .filter(|i| !i.is_empty())
            .collect();

        for (i, frame) in frames.iter().enumerate() {
            let frame_split: Vec<&str> = frame.split("|").collect();

            let time_delta: i64 = frame_split[0].parse().unwrap();

            if time_delta == -12345 && i == frames.len() - 1 {
                self.rng_seed = frame_split[3].parse().unwrap();
                continue;
            }

            let bits: u32 = frame_split[1].parse().unwrap();

            self.frames.push(ReplayFrame {
                time_delta: frame_split[0].parse().unwrap(),
                keys: KeyPressState::from_bits_truncate(bits),
            })
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ReplayFrame {
    pub time_delta: i64,
    pub keys: KeyPressState,
}
