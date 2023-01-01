#![allow(non_upper_case_globals)]
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::enums::GameMode;
use crate::maps::structures::*;
use crate::maps::QuaverMap;

#[derive(Default)]
pub struct OsuBeatmap {
    pub original_file_name: String,
    pub is_valid: bool,
    pub peppy_file_format: String,

    // [General]
    pub audio_file_name: String,
    pub audio_lead_in: i32,
    pub preview_time: i32,
    pub countdown: i32,
    pub sample_set: String,
    pub stack_leniency: f32,
    pub mode: i32,
    pub letterbox_in_breaks: i32,
    pub special_style: i32,
    pub widescreen_storyboard: i32,

    // [Editor]
    pub bookmarks: String,
    pub distance_spacing: f32,
    pub beat_divisor: i32,
    pub grid_size: i32,
    pub timeline_zoom: i32,

    // [Metadata]
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: String,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,

    // [Difficulty]
    pub hp_drain_rate: f32,
    pub key_count: i32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
    pub slider_multiplier: f32,
    pub slider_tick_rate: f32,

    // [Events]
    pub background: String,
    pub sound_effects: Vec<OsuSampleInfo>,

    // [TimingPoints]
    pub timing_points: Vec<OsuTimingPoint>,

    // [HitObjects]
    pub hit_objects: Vec<OsuHitObject>,

    pub custom_audio_samples: Vec<String>,
}

impl OsuBeatmap {
    pub fn from_path(file_path: &str) -> Self {
        let mut self_ = Self::default();

        if !Path::new(file_path).exists() {
            self_.is_valid = false;
        }

        self_.is_valid = true;
        self_.original_file_name = file_path.to_string();

        let mut section: &str = "";

        if let Ok(lines) = Self::read_lines(&self_.original_file_name) {
            for raw_line in lines {
                let raw_line = raw_line.unwrap();
                if raw_line.trim().is_empty()
                    || raw_line.starts_with("//")
                    || raw_line.starts_with(' ')
                    || raw_line.starts_with('_')
                {
                    continue;
                }

                let line = Self::strip_comments(raw_line.as_str());

                section = match line.trim() {
                    "[General]" => "[General]",
                    "[Editor]" => "[Editor]",
                    "[Metadata]" => "[Metadata]",
                    "[Difficulty]" => "[Difficulty]",
                    "[Events]" => "[Events]",
                    "[TimingPoints]" => "[TimingPoints]",
                    "[HitObjects]" => "[HitObjects]",
                    "[Colours]" => "[Colours]",
                    _ => section,
                };

                if line.starts_with("osu file format") {
                    self_.peppy_file_format = line.to_string();
                }

                if section.eq("[General]") && line.contains(':') {
                    let key = &line[..line.find(':').unwrap()];
                    let value = line.split(':').last().unwrap().trim();
                    match key.trim() {
                        "AudioFilename" => self_.audio_file_name = value.parse().unwrap(),
                        "AudioLeadIn" => self_.audio_lead_in = value.parse().unwrap(),
                        "PreviewTime" => self_.preview_time = value.parse().unwrap(),
                        "Countdown" => self_.countdown = value.parse().unwrap(),
                        "SampleSet" => self_.sample_set = value.parse().unwrap(),
                        "StackLeniency" => self_.stack_leniency = value.parse().unwrap(),
                        "Mode" => {
                            self_.mode = value.parse().unwrap();
                            if self_.mode != 3 {
                                self_.is_valid = false
                            }
                        }
                        "LetterboxInBreaks" => self_.letterbox_in_breaks = value.parse().unwrap(),
                        "SpecialStyle" => self_.special_style = value.parse().unwrap(),
                        "WidescreenStoryboard" => {
                            self_.widescreen_storyboard = value.parse().unwrap()
                        }
                        _ => (),
                    }
                }

                if section.eq("[Editor]") && line.contains(':') {
                    let key = &line[..line.find(':').unwrap()];
                    let value = line.split(':').last().unwrap().trim();

                    match key.trim() {
                        "Bookmarks" => self_.bookmarks = value.parse().unwrap(),
                        "DistanceSpacing" => self_.distance_spacing = value.parse().unwrap(),
                        "BeatDivisor" => self_.beat_divisor = value.parse().unwrap(),
                        "GridSize" => self_.grid_size = value.parse().unwrap(),
                        "TimelineZoom" => self_.timeline_zoom = value.parse().unwrap(),
                        _ => (),
                    }
                }

                if section.eq("[Metadata]") && line.contains(':') {
                    let key = &line[..line.find(':').unwrap()];
                    let value = line.split(':').last().unwrap().trim();

                    match key.trim() {
                        "Title" => self_.title = value.parse().unwrap(),
                        "TitleUnicode" => self_.title_unicode = value.parse().unwrap(),
                        "Artist" => self_.artist = value.parse().unwrap(),
                        "ArtistUnicode" => self_.artist_unicode = value.parse().unwrap(),
                        "Creator" => self_.creator = value.parse().unwrap(),
                        "Version" => self_.version = value.parse().unwrap(),
                        "Source" => self_.source = value.parse().unwrap(),
                        "Tags" => self_.tags = value.parse().unwrap(),
                        "BeatmapID" => self_.beatmap_id = value.parse().unwrap(),
                        "BeatmapSetID" => self_.beatmap_set_id = value.parse().unwrap(),
                        _ => (),
                    }
                }

                if section.eq("[Difficulty]") && line.contains(':') {
                    let key = &line[..line.find(':').unwrap()];
                    let value = line.split(':').last().unwrap().trim();

                    match key.trim() {
                        "HPDrainRate" => self_.hp_drain_rate = value.parse().unwrap(),
                        "CircleSize" => {
                            let key_count = value.parse().unwrap();

                            if key_count != 4 && key_count != 7 && key_count != 5 && key_count != 8
                            {
                                self_.is_valid = false;
                            }

                            self_.key_count = key_count;
                        }
                        "OverallDifficulty" => self_.overall_difficulty = value.parse().unwrap(),
                        "ApproachRate" => self_.approach_rate = value.parse().unwrap(),
                        "SliderMultiplier" => self_.slider_multiplier = value.parse().unwrap(),
                        "SliderTickRate" => self_.slider_tick_rate = value.parse().unwrap(),
                        _ => (),
                    }
                }

                if section.eq("[Events]") {
                    let values: Vec<&str> = line.split(',').collect();

                    if line.to_lowercase().contains("png")
                        || line.to_lowercase().contains("jpg")
                        || line.to_lowercase().contains("jpeg")
                    {
                        self_.background = values[2].replace('\"', "");
                    }

                    if values[0] == "Sample" || values[0] == "5" {
                        /*
                        let path = values[3]
                            .replace('"', "")
                            .replace(std::path::MAIN_SEPARATOR, "/");
                        */

                        self_.sound_effects.push(OsuSampleInfo {
                            start_time: values[1].parse().unwrap(),
                            layer: values[2].parse().unwrap(),
                            volume: (if values.len() >= 5 {
                                values[4].parse().unwrap()
                            } else {
                                100
                            })
                            .clamp(0, 100),
                            sample: 0,
                        })
                    }
                }

                if section.eq("[TimingPoints]") && line.contains(',') {
                    let values: Vec<&str> = line.split(',').collect();

                    let ms_per_beat: f32 = values[1].parse().unwrap();

                    let timing_point = OsuTimingPoint {
                        offset: values[0].parse().unwrap(),
                        milliseconds_per_beat: ms_per_beat,
                        // signature: if values[2] == "0" {
                        //     TimeSignature::Quadruple
                        // } else {
                        //     TimeSignature::from_bits(values[2].parse().unwrap()).unwrap()
                        // },
                        sample_type: values[3].parse().unwrap(),
                        sample_set: values[4].parse().unwrap(),
                        volume: values[5].parse().unwrap(),
                        inherited: values[6].parse().unwrap(),
                        kiai_mode: values[7].parse().unwrap(),
                    };

                    self_.timing_points.push(timing_point);
                }

                if section.eq("[HitObjects]") && line.contains(',') {
                    let values: Vec<&str> = line.split(',').collect();

                    let mut hit_object = OsuHitObject {
                        x: values[0].parse().unwrap(),
                        y: values[1].parse().unwrap(),
                        start_time: values[2].parse().unwrap(),
                        type_: HitObjectType::from_bits(values[3].parse().unwrap()).unwrap(),
                        hit_sound: HitSoundType::from_bits(values[4].parse().unwrap()).unwrap(),
                        additions: String::from("0:0:0:0:"),
                        key_sound: -1,
                        end_time: 0,
                        volume: 0,
                        ..Default::default()
                    };

                    if hit_object.type_ == HitObjectType::Hold {
                        let end_time = &values[5][..values[5].find(':').unwrap()];
                        hit_object.end_time = end_time.parse().unwrap();
                    }

                    if values.len() > 5 {
                        let additions: Vec<&str> = values[5].split(':').collect();

                        let volume_field = if hit_object.type_ == HitObjectType::Hold {
                            4
                        } else {
                            3
                        };

                        if additions.len() > volume_field && !additions[volume_field].is_empty() {
                            hit_object.volume =
                                std::cmp::max(0, additions[volume_field].parse().unwrap());
                        }

                        let key_sound_field = volume_field + 1;
                        if additions.len() > key_sound_field
                            && !additions[key_sound_field].is_empty()
                        {
                            /*
                            hit_object.key_sound = Self::custom_audio_sample_index(
                                &mut self_,
                                additions[key_sound_field],
                            )
                            */
                        }

                        self_.hit_objects.push(hit_object);
                    }
                }
            }
        }

        self_
    }

    pub fn to_qua(self) -> QuaverMap {
        let mut qua = QuaverMap {
            audio_file: self.audio_file_name,
            song_preview_time: self.preview_time,
            background_file: self.background,
            map_id: -1,
            map_set_id: -1,
            title: self.title,
            artist: self.artist,
            source: self.source,
            tags: self.tags,
            creator: self.creator,
            difficulty_name: self.version,
            description: String::from("This is a Quaver converted osu! map"),
            ..Default::default()
        };

        match self.key_count {
            4 => qua.mode = GameMode::Keys4,
            7 => qua.mode = GameMode::Keys7,
            8 => {
                qua.mode = GameMode::Keys7;
                qua.has_scratch_key = true;
            }
            _ => qua.mode = GameMode::Keys4,
        }

        for path in self.custom_audio_samples {
            qua.custom_audio_samples.push(CustomAudioSampleInfo {
                path,
                unaffected_by_rate: false,
            })
        }

        for info in self.sound_effects {
            if info.volume == 0 {
                continue;
            }

            qua.sound_effects.push(SoundEffectInfo {
                start_time: info.start_time as f32,
                sample: info.sample + 1,
                volume: info.volume,
            })
        }

        for tp in self.timing_points {
            let is_sv = tp.inherited == 0 || tp.milliseconds_per_beat < 0.;

            if is_sv {
                qua.slider_velocities.push(SliderVelocityInfo {
                    start_time: tp.offset,
                    multiplier: (-100. / tp.milliseconds_per_beat).clamp(0.1, 10.),
                })
            } else {
                qua.timing_points.push(TimingPointInfo {
                    start_time: tp.offset,
                    bpm: 60000. / tp.milliseconds_per_beat,
                    // signature: tp.signature,
                    ..Default::default()
                })
            }
        }

        for hit_object in self.hit_objects {
            let mut key_lane = (hit_object.x as f64 / (512f64 / self.key_count as f64))
                .clamp(0., (self.key_count - 1) as f64) as i32
                + 1;

            if qua.has_scratch_key {
                if key_lane == 1 {
                    key_lane = self.key_count;
                } else {
                    key_lane -= 1
                };
            }

            if hit_object.type_ == HitObjectType::Circle {
                qua.hit_objects.push(HitObjectInfo {
                    start_time: hit_object.start_time,
                    lane: key_lane,
                    end_time: 0,
                    // hit_sound: HitSounds::Normal, // TODO
                    // key_sounds: TODO
                })
            }
        }

        qua.sort();

        qua
    }

    #[allow(dead_code)]
    fn custom_audio_sample_index(&mut self, path: &str) -> i32 {
        for i in 0..self.custom_audio_samples.len() {
            if self.custom_audio_samples[i] == path {
                return i as i32;
            }
        }

        self.custom_audio_samples.push(path.to_string());
        self.custom_audio_samples.len() as i32 - 1
    }

    fn strip_comments(line: &str) -> &str {
        let index = line.find("//").unwrap_or(0);
        if index > 0 {
            return &line[..index];
        }
        line
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

bitflags! {
    #[derive(Default)]
    pub struct HitObjectType: i32{
        const Circle = 1 << 0;
        const Slider = 1 << 1;
        const NewCombo = 1 << 2;
        const Spinner = 1 << 3;
        const ComboOffset = 1 << 4 | 1 << 5 | 1 << 6;
        const Hold = 1 << 7;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct HitSoundType: u32 {
        const None = 0;
        const Normal = 1;
        const Whistle = 2;
        const Finish = 4;
        const Clap = 8;
    }
}

pub struct OsuTimingPoint {
    pub offset: f32,
    pub milliseconds_per_beat: f32,
    // pub signature: TimeSignature,
    pub sample_type: i32,
    pub sample_set: i32,
    pub volume: i32,
    pub inherited: i32,
    pub kiai_mode: i32,
}

#[derive(Default, Debug)]
pub struct OsuHitObject {
    pub x: i32,
    pub y: i32,
    pub start_time: i32,
    pub type_: HitObjectType,
    pub hit_sound: HitSoundType,
    pub end_time: i32,
    pub additions: String,
    pub key1: bool,
    pub key2: bool,
    pub key3: bool,
    pub key4: bool,
    pub key5: bool,
    pub key6: bool,
    pub key7: bool,
    pub volume: i32,
    pub key_sound: i32,
}

pub struct OsuSampleInfo {
    pub start_time: i32,
    pub layer: i32,
    pub volume: i32,
    pub sample: i32,
}
