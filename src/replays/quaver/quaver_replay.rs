use super::{ReplayAutoplayFrame, ReplayAutoplayFrameType, ReplayFrame, ReplayKeyPressState};
use crate::{enums::quaver::ModIdentifier, helpers::BinaryHelper, maps::QuaverMap};
use lzma_rs::lzma_decompress;
use semver::Version;
use std::collections::BTreeMap;
use std::fs;

#[derive(Default, Clone)]
pub struct QuaverReplay {
    pub replay_version: String,
    pub map_md5: String,
    pub md5: String,
    pub player_name: String,
    pub date: String,
    pub time_played: i64,
    pub mode: i32,
    pub mods: ModIdentifier,
    pub score: i32,
    pub accuracy: f32,
    pub max_combo: i32,
    pub count_marv: i32,
    pub count_perf: i32,
    pub count_great: i32,
    pub count_good: i32,
    pub count_okay: i32,
    pub count_miss: i32,
    pub pause_count: i32,
    pub frames: Vec<ReplayFrame>,
}

impl QuaverReplay {
    pub fn from_file(path: &str) -> Self {
        let mut self_ = Self {
            ..Default::default()
        };

        let data = fs::read(path).unwrap();
        let mut br = BinaryHelper::from_u8(&data);

        self_.replay_version = br.read_string().unwrap();
        self_.map_md5 = br.read_string().unwrap();
        self_.md5 = br.read_string().unwrap();
        self_.player_name = br.read_string().unwrap();
        self_.date = br.read_string().unwrap();
        self_.time_played = br.read_i64().unwrap();
        self_.mode = br.read_i32().unwrap();

        if self_.replay_version == "0.0.1" || self_.replay_version == "None" {
            let mut mods = br.read_i32().unwrap();
            if mods == -1 {
                self_.mods = ModIdentifier::None;
                mods = 0;
            } else if mods < 0 {
                self_.mods = ModIdentifier::Mirror;
                mods &= !(1 << 31);
            }
            self_.mods |= ModIdentifier::from_bits(mods as i64).unwrap();
        } else {
            self_.mods = ModIdentifier::from_bits(br.read_i64().unwrap()).unwrap();
        }

        self_.score = br.read_i32().unwrap();
        self_.accuracy = br.read_f32().unwrap();
        self_.max_combo = br.read_i32().unwrap();
        self_.count_marv = br.read_i32().unwrap();
        self_.count_perf = br.read_i32().unwrap();
        self_.count_great = br.read_i32().unwrap();
        self_.count_good = br.read_i32().unwrap();
        self_.count_okay = br.read_i32().unwrap();
        self_.count_miss = br.read_i32().unwrap();
        self_.pause_count = br.read_i32().unwrap();

        if self_.replay_version != "None" {
            let _replay_version = Version::parse(self_.replay_version.as_str()).unwrap();

            if _replay_version >= Version::parse("0.0.1").unwrap() {
                br.read_i32().unwrap();
            }
        }

        let length = br.remaining_length();
        let mut remaining_bytes = br.read_bytes(length).unwrap();
        let mut decomp: Vec<u8> = Vec::new();
        lzma_decompress(&mut remaining_bytes, &mut decomp).unwrap();

        let frames: Vec<String> = std::str::from_utf8(&decomp)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        for frame in frames {
            let frame_split: Vec<&str> = frame.split('|').collect();

            if frame_split.len() != 2 {
                continue;
            }

            self_.frames.push(ReplayFrame {
                time: frame_split[0].parse().unwrap(),
                keys: ReplayKeyPressState::from_bits(frame_split[1].parse().unwrap()).unwrap(),
            })
        }

        self_
    }

    pub fn generate_perfect_replay_keys(map: &mut QuaverMap) -> QuaverReplay {
        let mut non_combined: Vec<ReplayAutoplayFrame> = Vec::new();
        let mut replay = QuaverReplay::default();

        for &hit_object in map.hit_objects.iter() {
            non_combined.push(ReplayAutoplayFrame {
                hit_object,
                type_: ReplayAutoplayFrameType::Press,
                time: hit_object.start_time,
                keys: Self::key_lane_to_press_state(hit_object.lane).unwrap(),
            });

            if hit_object.end_time > 0 {
                non_combined.push(ReplayAutoplayFrame {
                    hit_object,
                    type_: ReplayAutoplayFrameType::Release,
                    time: hit_object.end_time - 1,
                    keys: Self::key_lane_to_press_state(hit_object.lane).unwrap(),
                })
            } else {
                non_combined.push(ReplayAutoplayFrame {
                    hit_object,
                    type_: ReplayAutoplayFrameType::Release,
                    time: hit_object.start_time + 30,
                    keys: Self::key_lane_to_press_state(hit_object.lane).unwrap(),
                })
            }
        }

        non_combined.sort_by_key(|x| x.time);

        let mut state: ReplayKeyPressState = ReplayKeyPressState::from_bits(0).unwrap();

        replay.frames.push(ReplayFrame {
            time: -10000,
            keys: ReplayKeyPressState::from_bits(0).unwrap(),
        });

        let grouped = Self::group_auto_frames(&non_combined);

        for (time, val) in grouped {
            for frame in val.iter() {
                match frame.type_ {
                    ReplayAutoplayFrameType::Press => state |= frame.keys,
                    ReplayAutoplayFrameType::Release => state &= !frame.keys,
                }
            }

            replay.frames.push(ReplayFrame { time, keys: state })
        }

        replay
    }

    fn key_lane_to_press_state(lane: i32) -> Option<ReplayKeyPressState> {
        ReplayKeyPressState::from_bits(1 << (lane - 1))
    }

    fn group_auto_frames(
        vec: &Vec<ReplayAutoplayFrame>,
    ) -> BTreeMap<i32, Vec<ReplayAutoplayFrame>> {
        let mut number_groups: BTreeMap<i32, Vec<ReplayAutoplayFrame>> = BTreeMap::new();
        for &n in vec {
            number_groups.entry(n.time).or_default().push(n);
        }

        number_groups
    }
}
