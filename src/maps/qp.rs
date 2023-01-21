use crate::maps::QuaverMap;
use anyhow::Result;
use std::{collections::HashMap, io::Read};
use zip;

#[derive(Debug)]
pub struct QuaverMapset {
    pub maps: HashMap<String, QuaverMap>,
    pub audio: Vec<u8>,
}

impl QuaverMapset {
    pub fn from_path(path: &str) -> Result<Self> {
        let mut self_ = Self {
            maps: HashMap::new(),
            audio: Vec::new(),
        };

        let qp_file = std::fs::File::open(path)?;

        let zip = zip::ZipArchive::new(&qp_file)?;

        for name in zip.file_names() {
            let mut zip = zip.clone();
            if name.contains(".qua") {
                let mut contents = String::new();
                let mut qua_file = zip.by_name(name)?;
                qua_file.read_to_string(&mut contents)?;
                let md5 = format!("{:x}", md5::compute(&contents));
                self_.maps.insert(md5, QuaverMap::from_string(&contents));
            } else if name.contains(".mp3") {
                let mut audio_file = zip.by_name(name)?;
                let mut buffer: Vec<u8> = Vec::new();
                audio_file.read_to_end(&mut buffer)?;
                self_.audio = buffer
            }
        }

        Ok(self_)
    }
}
