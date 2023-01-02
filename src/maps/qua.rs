use super::structures::*;
use crate::enums::quaver::{GameMode, ModIdentifier};

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

    pub fn apply_mods(&mut self, mods: ModIdentifier) {
        if mods.contains(ModIdentifier::NoLongNotes) {
            self.replace_long_notes_with_regular_notes();
        }

        if mods.contains(ModIdentifier::Inverse) {
            self.apply_inverse();
        }

        if mods.contains(ModIdentifier::FullLN) {
            self.replace_long_notes_with_regular_notes();
            self.apply_inverse();
        }

        if mods.contains(ModIdentifier::Mirror) {
            self.mirror_hit_objects();
        }
    }

    fn get_timing_point_at(&self, time: f64) -> Option<TimingPointInfo> {
        let index = self
            .timing_points
            .iter()
            .enumerate()
            .rfind(|(_, x)| x.start_time as f64 <= time)
            .map(|(i, _)| i);

        if index.is_none() {
            return if self.timing_points.is_empty() {
                None
            } else {
                Some(self.timing_points.first().unwrap().clone())
            };
        };

        Some(self.timing_points[index.unwrap()].clone())
    }

    fn apply_inverse(&mut self) {
        const MINIMAL_LN_LENGTH: i32 = 36;
        const MINIMAL_GAP_LENGTH: i32 = 36;

        let mut new_hit_objects: Vec<HitObjectInfo> = Vec::new();

        let mut first_in_lane = vec![true; self.get_key_count(None) as usize];

        for i in 0..first_in_lane.len() - 1 {
            first_in_lane[i] = true;
        }

        let mut next_object_in_lane: Option<HitObjectInfo> = None;
        let mut second_next_object_in_lane: Option<HitObjectInfo> = None;
        for i in 0..self.hit_objects.len() {
            let mut current_object = self.hit_objects[i];
            for j in (i + 1)..self.hit_objects.len() {
                if self.hit_objects[j].lane == current_object.lane {
                    if next_object_in_lane.is_none() {
                        next_object_in_lane = Some(self.hit_objects[j]);
                    } else {
                        second_next_object_in_lane = Some(self.hit_objects[j]);
                        break;
                    }
                }
            }

            let is_first_in_lane = first_in_lane[current_object.lane as usize - 1];
            first_in_lane[current_object.lane as usize - 1] = false;

            if next_object_in_lane.is_none() && is_first_in_lane {
                new_hit_objects.push(current_object);
                continue;
            }

            let mut time_gap: Option<i32> = None;
            if let Some(next_object_in_lane) = next_object_in_lane {
                let timing_point = self
                    .get_timing_point_at(next_object_in_lane.start_time as f64)
                    .unwrap();

                let bpm = if timing_point.start_time == next_object_in_lane.start_time as f32 {
                    let prev_timing_point_index = self
                        .timing_points
                        .iter()
                        .enumerate()
                        .rfind(|(_, x)| x.start_time < timing_point.start_time)
                        .map(|(i, _)| i);

                    self.timing_points[prev_timing_point_index.unwrap_or(0)].bpm
                } else {
                    timing_point.bpm
                };

                time_gap = Some(i32::max(
                    f32::round(15000. / bpm) as i32,
                    MINIMAL_GAP_LENGTH,
                ));
            };

            if current_object.is_long_note() {
                if let Some(next_object_in_lane) = next_object_in_lane {
                    current_object.start_time = current_object.end_time;
                    current_object.end_time =
                        next_object_in_lane.start_time - time_gap.unwrap_or(0);

                    if second_next_object_in_lane.is_none() != next_object_in_lane.is_long_note() {
                        current_object.end_time = next_object_in_lane.start_time
                    }

                    if current_object.end_time - current_object.start_time < MINIMAL_LN_LENGTH {
                        continue;
                    }
                }
            } else {
                if next_object_in_lane.is_none() {
                    continue;
                }

                current_object.end_time =
                    next_object_in_lane.unwrap().start_time - time_gap.unwrap_or(0);

                if second_next_object_in_lane.is_none()
                    == (next_object_in_lane.unwrap().end_time == 0)
                {
                    current_object.end_time = next_object_in_lane.unwrap().start_time;
                }

                if current_object.end_time - current_object.start_time < MINIMAL_LN_LENGTH {
                    current_object.end_time = 0;
                }
            }

            new_hit_objects.push(current_object)
        }

        new_hit_objects.sort_by(|a, b| a.start_time.cmp(&b.start_time));

        self.hit_objects = new_hit_objects;
    }

    fn mirror_hit_objects(&mut self) {
        let key_count = self.get_key_count(None);
        for hit_object in self.hit_objects.iter_mut() {
            hit_object.lane = key_count - hit_object.lane + 1;
        }
    }

    fn replace_long_notes_with_regular_notes(&mut self) {
        for hit_object in self.hit_objects.iter_mut() {
            hit_object.end_time = 0
        }
    }
}
