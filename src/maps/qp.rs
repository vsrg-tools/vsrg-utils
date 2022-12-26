use crate::maps::QuaverMap;
use std::{collections::HashMap, io::Read};
use zip;

#[derive(Debug)]
pub struct QuaverMapset {
    pub maps: HashMap<String, QuaverMap>,
}

impl QuaverMapset {
    pub fn from_path(path: &str) -> Self {
        let mut self_ = Self {
            maps: HashMap::new(),
        };

        let qp_file = std::fs::File::open(path).unwrap();

        let zip = zip::ZipArchive::new(&qp_file).unwrap();

        for name in zip.file_names() {
            if name.contains(".qua") {
                let mut contents = String::new();
                let mut zip = zip::ZipArchive::new(&qp_file).unwrap();
                let mut qua_file = zip.by_name(name).unwrap();
                qua_file.read_to_string(&mut contents).unwrap();
                let md5 = format!("{:x}", md5::compute(&contents));
                self_.maps.insert(md5, QuaverMap::from_string(&contents));
            }
        }

        self_
    }
}
