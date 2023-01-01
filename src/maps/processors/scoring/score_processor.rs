use num_traits::cast::FromPrimitive;
use std::collections::{BTreeMap, HashMap};

use crate::{enums::quaver::ModIdentifier, maps::QuaverMap, replays::QuaverReplay};

use super::{
    data::{HitStat, KeyPressType},
    judgement_windows::{Judgement, JudgementWindows},
};

#[derive(Default)]
pub struct ScoreProcessor {
    pub map: QuaverMap,
    pub mods: ModIdentifier,
    pub score: i64,
    pub accuracy: f32,
    pub health: f32,
    pub combo: i64,
    pub max_combo: i64,
    pub windows: JudgementWindows,
    pub stats: Vec<HitStat>,
    force_fail: bool,
    total_judgements: i32,
    pub summed_score: i64,
    multiplier_count: i64,
    multiplier_index: i64,
    pub current_judgements: BTreeMap<Judgement, i32>,
    pub judgement_window: HashMap<Judgement, f32>,
    pub score_count: i64,
    pub player_name: String,
    pub date: String,
    judgement_health_weighting: HashMap<Judgement, f32>,
}

impl ScoreProcessor {
    #[allow(dead_code)]
    const VERSION: &'static str = "0.0.1";

    pub fn windows_release_multiplier() -> HashMap<Judgement, f32> {
        HashMap::from([
            (Judgement::Marv, 1.5),
            (Judgement::Perf, 1.5),
            (Judgement::Great, 1.5),
            (Judgement::Good, 1.5),
            (Judgement::Okay, 1.5),
            (Judgement::Miss, 1.5),
        ])
    }

    pub fn judgement_score_weighting() -> HashMap<Judgement, i32> {
        HashMap::from([
            (Judgement::Marv, 100),
            (Judgement::Perf, 50),
            (Judgement::Great, 25),
            (Judgement::Good, 10),
            (Judgement::Okay, 5),
            (Judgement::Miss, 0),
        ])
    }
    pub fn judgement_health_weighting(&mut self) -> &mut HashMap<Judgement, f32> {
        if self.judgement_health_weighting.is_empty() {
            self.judgement_health_weighting = HashMap::from([
                (Judgement::Marv, 0.5),
                (Judgement::Perf, 0.4),
                (Judgement::Great, 0.2),
                (Judgement::Good, -3.),
                (Judgement::Okay, -4.5),
                (Judgement::Miss, -6.),
            ]);
        }
        &mut self.judgement_health_weighting
    }

    pub fn judgement_accuracy_weighting() -> HashMap<Judgement, f32> {
        HashMap::from([
            (Judgement::Marv, 100.),
            (Judgement::Perf, 98.25),
            (Judgement::Great, 65.),
            (Judgement::Good, 25.),
            (Judgement::Okay, -100.),
            (Judgement::Miss, -50.),
        ])
    }

    const MULTIPLIER_MAX_INDEX: i32 = 15;
    const MULTIPLIER_COUNT_TO_INCREASE_INDEX: i32 = 10;
    const STANDARDIZED_MAX_SCORE: i32 = 1_000_000;

    pub fn failed(&self) -> bool {
        self.health <= 0. && !self.mods.contains(ModIdentifier::NoFail) || self.force_fail
    }

    pub fn force_fail(&self) -> bool {
        self.force_fail
    }

    fn max_multiplier_count() -> i64 {
        (Self::MULTIPLIER_MAX_INDEX * Self::MULTIPLIER_COUNT_TO_INCREASE_INDEX) as i64
    }

    pub fn total_judgement_count(&self) -> i32 {
        let mut sum: i32 = 0;
        for (_, value) in self.current_judgements.iter() {
            sum += value;
        }
        sum
    }

    pub fn from_map(
        map: &QuaverMap,
        mods: ModIdentifier,
        windows: Option<JudgementWindows>,
    ) -> Self {
        let map_clone = map.clone();
        let mut self_ = Self {
            map: map_clone,
            mods,
            health: 100.,
            ..Default::default()
        };
        self_.current_judgements = BTreeMap::from([
            (Judgement::Marv, 0),
            (Judgement::Perf, 0),
            (Judgement::Great, 0),
            (Judgement::Good, 0),
            (Judgement::Okay, 0),
            (Judgement::Miss, 0),
        ]);
        self_.initialise_judgement_windows(windows);
        // TODO: self_.initialise_mods();
        self_.total_judgements = self_.get_total_judgement_count();
        self_.summed_score = self_.calculate_summed_score();
        self_.initialise_health_weighting();
        self_
    }

    pub fn from_replay(replay: &QuaverReplay, windows: Option<JudgementWindows>) -> Self {
        let mut self_ = Self {
            health: 100.,
            mods: replay.mods,
            score: replay.score as i64,
            accuracy: replay.accuracy,
            max_combo: replay.max_combo as i64,
            player_name: replay.player_name.clone(),
            date: replay.date.clone(),
            judgement_window: HashMap::from([
                (Judgement::Marv, 18.),
                (Judgement::Perf, 43.),
                (Judgement::Great, 76.),
                (Judgement::Good, 106.),
                (Judgement::Okay, 127.),
                (Judgement::Miss, 164.),
            ]),
            ..Default::default()
        };

        self_
            .current_judgements
            .insert(Judgement::Marv, replay.count_marv);
        self_
            .current_judgements
            .insert(Judgement::Perf, replay.count_perf);
        self_
            .current_judgements
            .insert(Judgement::Great, replay.count_great);
        self_
            .current_judgements
            .insert(Judgement::Good, replay.count_good);
        self_
            .current_judgements
            .insert(Judgement::Okay, replay.count_okay);
        self_
            .current_judgements
            .insert(Judgement::Miss, replay.count_miss);

        self_.initialise_judgement_windows(windows);
        // self_.initialise_mods();

        self_
    }

    fn initialise_judgement_windows(&mut self, windows: Option<JudgementWindows>) {
        self.windows = windows.unwrap_or(JudgementWindows {
            name: "Standard".to_string(),
            is_default: true,
            ..Default::default()
        });
        self.judgement_window
            .insert(Judgement::Marv, self.windows.marvelous);
        self.judgement_window
            .insert(Judgement::Perf, self.windows.perfect);
        self.judgement_window
            .insert(Judgement::Great, self.windows.great);
        self.judgement_window
            .insert(Judgement::Good, self.windows.good);
        self.judgement_window
            .insert(Judgement::Okay, self.windows.okay);
        self.judgement_window
            .insert(Judgement::Miss, self.windows.miss);
    }

    // TODO: GetRateFromMods(Mods)
    // fn initialise_mods(&mut self) {
    //     for i in 0..self.judgement_window.len() {
    //         self.judgement_window[&Judgement::VALUES[i]] = GetRateFromMods(Mods);
    //     }
    // }

    pub fn calculate_score_from_hit_difference(
        &mut self,
        hit_difference: i32,
        key_press_type: KeyPressType,
        calculate_all_stats: Option<bool>,
    ) -> Judgement {
        let calculate_all_stats = calculate_all_stats.unwrap_or(true);

        if hit_difference == i32::MIN {
            return Judgement::Miss;
        }

        let absolute_difference = hit_difference.abs();

        let mut judgement = Judgement::Ghost;

        for i in 0..self.judgement_window.len() {
            let j = Judgement::from_usize(i).unwrap();
            if key_press_type == KeyPressType::RELEASE && j == Judgement::Miss {
                break;
            }

            let window = if key_press_type == KeyPressType::RELEASE {
                self.judgement_window[&j] * Self::windows_release_multiplier()[&j]
            } else {
                self.judgement_window[&j]
            };

            if !(absolute_difference as f32 <= window) {
                continue;
            }

            if key_press_type == KeyPressType::RELEASE && j == Judgement::Okay {
                judgement = Judgement::Good;
                break;
            }

            judgement = j;
            break;
        }

        if judgement == Judgement::Ghost {
            return judgement;
        }

        if calculate_all_stats {
            self.calculate_score_from_judgement(
                judgement,
                Some(key_press_type == KeyPressType::RELEASE),
            );
        }

        judgement
    }

    pub fn calculate_score_from_judgement(
        &mut self,
        judgement: Judgement,
        is_long_note_release: Option<bool>,
    ) {
        let _is_long_note_release = is_long_note_release.unwrap_or(false);

        *self.current_judgements.entry(judgement).or_insert(0) += 1;
        self.accuracy = self.calculate_accuracy();

        // Score calculation.
        let mut combo_break_judgement = self.windows.combo_break_judgement;

        if combo_break_judgement == Judgement::Marv || combo_break_judgement == Judgement::Ghost {
            combo_break_judgement = Judgement::Miss;
        }

        // If the user didn't miss, then we want to update their combo and multiplier.
        if judgement < combo_break_judgement {
            // Update multiplier.
            if judgement == Judgement::Good {
                self.multiplier_count -= Self::MULTIPLIER_COUNT_TO_INCREASE_INDEX as i64;
            } else {
                self.multiplier_count += 1;
            }

            self.combo += 1;

            if self.combo > self.max_combo {
                self.max_combo = self.combo;
            }
        } else {
            // The user missed, so we want to decrease their multipler by 2 indexes and reset their combo.
            self.multiplier_count -= Self::MULTIPLIER_COUNT_TO_INCREASE_INDEX as i64 * 2;
            self.combo = 0;

            // This is probably unnecessary.
            if self.mods.contains(ModIdentifier::NoMiss) {
                self.health = 0.;
                self.force_fail = true;
                return;
            }
        }

        // Make sure the multiplier count doesn't go below 0 nor over max multiplier count.
        self.multiplier_count = self.multiplier_count.clamp(0, Self::max_multiplier_count());

        // Update multiplier index and score count.
        self.multiplier_index = (self.multiplier_count as f32
            / Self::MULTIPLIER_COUNT_TO_INCREASE_INDEX as f32)
            .floor() as i64;
        self.score_count += Self::judgement_score_weighting()[&judgement] as i64
            + self.multiplier_index * Self::MULTIPLIER_COUNT_TO_INCREASE_INDEX as i64;

        // Update total score.
        self.score = (Self::STANDARDIZED_MAX_SCORE as f64
            * (self.score_count as f64 / self.summed_score as f64)) as i64;

        // Health calculation.
        self.health += self.judgement_health_weighting()[&judgement];
        self.health = self.health.clamp(0., 100.);
    }

    fn calculate_accuracy(&self) -> f32 {
        let mut acc = 0.;

        for (&judgement, &value) in self.current_judgements.iter() {
            acc += value as f32 * Self::judgement_accuracy_weighting()[&judgement];
        }

        0f32.max(
            acc / (self.total_judgement_count() as f32
                * Self::judgement_accuracy_weighting()[&Judgement::Marv]),
        ) * Self::judgement_accuracy_weighting()[&Judgement::Marv]
    }

    pub fn get_total_judgement_count(&self) -> i32 {
        let mut judgements = 0;
        for o in self.map.hit_objects.iter() {
            if o.end_time > 0 {
                judgements += 2;
            } else {
                judgements += 1;
            }
        }

        judgements
    }

    fn calculate_summed_score(&self) -> i64 {
        let mut summed_score: i64 = 0;

        let max_multiplier_count =
            Self::MULTIPLIER_MAX_INDEX * Self::MULTIPLIER_COUNT_TO_INCREASE_INDEX;

        for i in 1..(self.total_judgements + 1).min(max_multiplier_count) {
            summed_score += Self::judgement_score_weighting()[&Judgement::Marv] as i64
                + Self::MULTIPLIER_COUNT_TO_INCREASE_INDEX as i64
                    * (i as f32 / Self::MULTIPLIER_COUNT_TO_INCREASE_INDEX as f32).floor() as i64
        }

        if self.total_judgements >= max_multiplier_count {
            summed_score += (self.total_judgements as i64 - (max_multiplier_count as i64 - 1))
                * (Self::judgement_score_weighting()[&Judgement::Marv] as i64
                    + max_multiplier_count as i64);
        }

        summed_score
    }

    fn initialise_health_weighting(&mut self) {
        if self.mods.contains(ModIdentifier::Autoplay) {
            return;
        }

        // TODO: get mods multiplier
        let mut density = self.map.get_actions_per_second(None);

        if density == 0. || density >= 12. || density.is_nan() {
            return;
        }

        // base is 2
        density = density.max(2.);

        let values = HashMap::from([
            (Judgement::Marv, (-0.14, 2.68)),
            (Judgement::Perf, (-0.2, 3.4)),
            (Judgement::Great, (-0.14, 2.68)),
            (Judgement::Good, (0.084, -0.008)),
            (Judgement::Okay, (0.081, -0.028)),
        ]);

        for (key, val) in values.iter() {
            let multiplier = val.0 * density + val.1;
            let weight = self.judgement_health_weighting()[key];
            self.judgement_health_weighting()
                .insert(*key, (multiplier * weight * 100.).round() / 100.);
        }
    }
}
