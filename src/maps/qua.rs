use super::structures::*;
use crate::enums::GameMode;

use serde::{Deserialize, Serialize};

fn constant_1() -> f32 {
    1.0
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct QuaverMap {
    pub audio_file: String,
    pub song_preview_time: i32,
    pub background_file: String,
    pub banner_file: String,
    pub map_id: i32,
    pub map_set_id: i32,
    pub mode: GameMode,
    pub title: String,
    pub artist: String,
    pub source: String,
    pub tags: String,
    pub creator: String,
    pub difficulty_name: String,
    pub description: String,
    pub genre: String,
    pub bpm_does_not_affect_scroll_velocity: bool,
    #[serde(default = "constant_1")]
    pub initial_scroll_velocity: f32,
    pub has_scratch_key: bool,
    pub editor_layers: Vec<EditorLayerInfo>,
    pub custom_audio_samples: Vec<CustomAudioSampleInfo>,
    pub sound_effects: Vec<SoundEffectInfo>,
    pub timing_points: Vec<TimingPointInfo>,
    pub slider_velocities: Vec<SliderVelocityInfo>,
    pub hit_objects: Vec<HitObjectInfo>,
    pub file_path: String,
}

impl QuaverMap {
    pub fn from_path(path: &str) -> Self {
        let file = std::fs::File::open(path).unwrap();
        serde_yaml::from_reader(file).unwrap()
    }

    pub fn from_string(input: &str) -> Self {
        serde_yaml::from_str(input).unwrap()
    }

    pub fn sort(&mut self) {
        self.hit_objects.sort_by_key(|x| x.start_time);
        self.timing_points.sort_by_key(|x| x.start_time as i32);
        self.slider_velocities.sort_by_key(|x| x.start_time as i32);
        self.sound_effects.sort_by_key(|x| x.start_time as i32);
    }

    pub fn to_file(&self) -> Result<(), serde_yaml::Error> {
        let w = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}.QuaverMap", self.title))
            .expect("unable to open file");

        serde_yaml::to_writer(w, self)
    }

    pub fn length(&self) -> i32 {
        if self.hit_objects.is_empty() {
            0
        } else {
            self.hit_objects
                .iter()
                .map(|x| std::cmp::max(x.start_time, x.end_time))
                .max()
                .unwrap()
        }
    }

    pub fn get_key_count(&self, include_scratch: Option<bool>) -> i32 {
        let mut count: i32;

        match self.mode {
            GameMode::Keys4 => count = 4,
            GameMode::Keys7 => count = 7,
        }

        if self.has_scratch_key && include_scratch.unwrap_or(true) {
            count += 1;
        }

        count
    }

    pub fn get_actions_per_second(&self, rate: Option<f32>) -> f32 {
        let rate = rate.unwrap_or(1.);
        let mut actions: Vec<i32> = Vec::new();

        for &info in self.hit_objects.iter() {
            actions.push(info.start_time);

            if info.end_time > 0 {
                actions.push(info.end_time);
            }
        }

        if actions.is_empty() {
            return 0.;
        }

        actions.sort();

        let mut length = actions.last().unwrap() - actions.first().unwrap();

        for i in 0..actions.len() {
            if i == 0 {
                continue;
            }

            let action = actions[i];
            let previous_action = actions[i - 1];
            let difference = action - previous_action;

            if difference >= 1000 {
                length -= difference;
            }
        }

        actions.len() as f32 / (length as f32 / (1000. * rate))
    }

    #[allow(dead_code)]
    fn get_hit_object_at_judgement_index(&self, index: i32) -> Option<HitObjectInfo> {
        let mut h: Option<HitObjectInfo> = None;
        let mut total = 0;

        for &hit_object in self.hit_objects.iter() {
            total += 1;

            if total - 1 == index {
                h = Some(hit_object);
                break;
            }

            if hit_object.end_time > 0 {
                total += 1;
            }

            if total - 1 == index {
                h = Some(hit_object);
                break;
            }
        }

        h
    }
}
