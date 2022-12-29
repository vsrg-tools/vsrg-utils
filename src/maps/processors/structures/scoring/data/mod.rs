use crate::maps::structures::HitObjectInfo;

use super::judgement_windows::Judgement;

#[derive(Copy, Clone)]
pub struct HitStat {
    pub type_: HitStatType,
    pub key_press_type: KeyPressType,
    pub hit_object: Option<HitObjectInfo>,
    pub song_position: i32,
    pub judgement: Judgement,
    pub hit_difference: i32,
    pub accuracy: f32,
    pub health: f32,
}

impl HitStat {
    pub fn new(
        type_: HitStatType,
        key_press_type: KeyPressType,
        hit_difference: i32,
        hit_object: Option<HitObjectInfo>,
        song_position: Option<i32>,
        judgement: Option<Judgement>,
        accuracy: Option<f32>,
        health: Option<f32>,
    ) -> Self {
        Self {
            type_,
            key_press_type,
            hit_difference,
            hit_object,
            song_position: song_position.unwrap_or(0),
            judgement: judgement.unwrap_or(Judgement::Ghost),
            accuracy: accuracy.unwrap_or(0.0),
            health: health.unwrap_or(0.0),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum HitStatType {
    HIT,
    MISS,
}

#[derive(PartialEq, Copy, Clone)]
pub enum KeyPressType {
    NONE,
    PRESS,
    RELEASE,
}
