use crate::enums::quaver::{GameMode, ModIdentifier, QssPatternFlags};
use crate::maps::QuaverMap;
use std::collections::HashMap;

use super::structures::{
    FingerAction, FingerState, Hand, LnLayerType, StrainSolverData, StrainSolverHitObject,
};
use super::StrainConstants;

#[derive(Default, Clone)]
pub struct DifficultyProcessor {
    pub map: QuaverMap,
    pub overall_difficulty: f32,
    pub qss_pattern_flags: QssPatternFlags,
    pub strain_constants: StrainConstants,
    pub average_note_density: f32,
    pub strain_solver_data: Vec<StrainSolverData>,
    lane_to_hand_4k: HashMap<i32, Hand>,
    lane_to_hand_7k: HashMap<i32, Hand>,
    lane_to_finger_4k: HashMap<i32, FingerState>,
    lane_to_finger_7k: HashMap<i32, FingerState>,
    pub vibro_inaccuracy_confidence: f32,
    roll_inaccuracy_confidence: f32,
}

impl DifficultyProcessor {
    pub const VERSION: &'static str = "0.0.5";

    pub fn new(
        map: &QuaverMap,
        constants: StrainConstants,
        mods: Option<ModIdentifier>,
        detailed_solve: Option<bool>,
    ) -> Self {
        let mut self_ = Self {
            lane_to_hand_4k: HashMap::from([
                (1, Hand::Left),
                (2, Hand::Left),
                (3, Hand::Right),
                (4, Hand::Right),
            ]),
            lane_to_hand_7k: HashMap::from([
                (1, Hand::Left),
                (2, Hand::Left),
                (3, Hand::Left),
                (4, Hand::Ambiguous),
                (5, Hand::Right),
                (6, Hand::Right),
                (7, Hand::Right),
            ]),
            lane_to_finger_4k: HashMap::from([
                (1, FingerState::Middle),
                (2, FingerState::Index),
                (3, FingerState::Index),
                (4, FingerState::Middle),
            ]),
            lane_to_finger_7k: HashMap::from([
                (1, FingerState::Ring),
                (2, FingerState::Middle),
                (3, FingerState::Index),
                (4, FingerState::Thumb),
                (5, FingerState::Index),
                (6, FingerState::Middle),
                (7, FingerState::Ring),
            ]),
            ..Default::default()
        };

        self_.strain_constants = constants;
        self_.map = map.clone();

        if self_.map.hit_objects.len() < 2 {
            return self_;
        }

        self_.calculate_difficulty(mods.unwrap_or(ModIdentifier::None));

        if detailed_solve.unwrap_or_default() {
            self_.compute_for_pattern_flags();
        }

        self_
    }

    fn calculate_difficulty(&mut self, mods: ModIdentifier) {
        let rate = mods.rate();

        match self.map.mode {
            GameMode::Keys4 => {
                self.overall_difficulty = self.compute_for_overall_difficulty(rate, None)
            }
            GameMode::Keys7 => {
                self.overall_difficulty = (self
                    .compute_for_overall_difficulty(rate, Some(Hand::Left))
                    + self.compute_for_overall_difficulty(rate, Some(Hand::Right)))
                    / 2.
            }
        }
    }

    fn compute_for_overall_difficulty(&mut self, rate: f32, assume_hand: Option<Hand>) -> f32 {
        self.compute_note_density_data(rate);
        self.compute_base_strain_states(rate, assume_hand.unwrap_or_default());
        self.compute_for_chords();
        self.compute_for_finger_actions();
        self.compute_for_roll_manipulation();
        self.compute_for_jack_manipulation();
        self.compute_for_ln_multiplier();
        self.calculate_overall_difficulty()
    }

    fn compute_base_strain_states(&mut self, rate: f32, assume_hand: Hand) {
        for &hit_object in self.map.hit_objects.iter() {
            if self.map.has_scratch_key && hit_object.lane == self.map.get_key_count(None) {
                continue;
            }

            let mut cur_hit_ob = StrainSolverHitObject::new(hit_object);
            let mut cur_strain_data: StrainSolverData;

            match self.map.mode {
                GameMode::Keys4 => {
                    cur_hit_ob.finger_state = self.lane_to_finger_4k[&hit_object.lane];
                    cur_strain_data = StrainSolverData::new(cur_hit_ob, Some(rate));
                    cur_strain_data.hand = self.lane_to_hand_4k[&hit_object.lane];
                }
                GameMode::Keys7 => {
                    cur_hit_ob.finger_state = self.lane_to_finger_7k[&hit_object.lane];
                    cur_strain_data = StrainSolverData::new(cur_hit_ob, Some(rate));
                    cur_strain_data.hand =
                        if self.lane_to_hand_7k[&hit_object.lane] == Hand::Ambiguous {
                            assume_hand
                        } else {
                            self.lane_to_hand_7k[&hit_object.lane]
                        }
                }
            }

            self.strain_solver_data.push(cur_strain_data);
        }
    }

    fn compute_for_chords(&mut self) {
        // had to do this because the size changes
        for i in 0.. {
            if i >= self.strain_solver_data.len() - 1 {
                break;
            }
            for j in i + 1.. {
                if j >= self.strain_solver_data.len() {
                    break;
                }

                let ms_diff =
                    self.strain_solver_data[j].start_time - self.strain_solver_data[i].start_time;

                if ms_diff > self.strain_constants.chord_clump_tolerance_ms {
                    break;
                }

                if ms_diff.abs() <= self.strain_constants.chord_clump_tolerance_ms
                    && self.strain_solver_data[i].hand == self.strain_solver_data[j].hand
                {
                    for k in self.strain_solver_data[j].hit_objects.clone() {
                        let mut same_state_found = false;
                        for l in self.strain_solver_data[i].hit_objects.iter() {
                            if l.finger_state == k.finger_state {
                                same_state_found = true;
                            }
                        }

                        if !same_state_found {
                            self.strain_solver_data[i].hit_objects.push(k);
                        }
                    }

                    self.strain_solver_data.remove(j);
                }
            }
        }

        for qss_data in self.strain_solver_data.iter_mut() {
            qss_data.solve_finger_state();
        }
    }

    fn compute_for_finger_actions(&mut self) {
        for i in 0..self.strain_solver_data.len() - 1 {
            for j in (i + 1)..self.strain_solver_data.len() {
                if self.strain_solver_data[i].hand == self.strain_solver_data[j].hand
                    && self.strain_solver_data[j].start_time > self.strain_solver_data[i].start_time
                {
                    let action_jack_found = (self.strain_solver_data[j].finger_state
                        & self.strain_solver_data[i].finger_state)
                        != FingerState::None;

                    let action_chord_found = self.strain_solver_data[i].hand_chord()
                        || self.strain_solver_data[j].hand_chord();

                    let action_same_state = self.strain_solver_data[i].finger_state
                        == self.strain_solver_data[j].finger_state;

                    let action_duration = self.strain_solver_data[j].start_time
                        - self.strain_solver_data[i].start_time;

                    self.strain_solver_data[i].next_strain_solver_data_on_current_hand = Some(j);
                    self.strain_solver_data[i].finger_action_duration_ms = action_duration;

                    if !action_chord_found && !action_same_state {
                        self.strain_solver_data[i].finger_action = FingerAction::Roll;
                        self.strain_solver_data[i].action_strain_coefficient = self
                            .get_coefficient_value(
                                action_duration,
                                self.strain_constants.roll_lower_boundary_ms,
                                self.strain_constants.roll_upper_boundary_ms,
                                self.strain_constants.roll_max_strain_value,
                                self.strain_constants.roll_curve_exponential,
                            );
                    } else if action_same_state {
                        self.strain_solver_data[i].finger_action = FingerAction::SimpleJack;
                        self.strain_solver_data[i].action_strain_coefficient = self
                            .get_coefficient_value(
                                action_duration,
                                self.strain_constants.s_jack_lower_boundary_ms,
                                self.strain_constants.s_jack_upper_boundary_ms,
                                self.strain_constants.s_jack_max_strain_value,
                                self.strain_constants.s_jack_curve_exponential,
                            );
                    } else if action_jack_found {
                        self.strain_solver_data[i].finger_action = FingerAction::TechnicalJack;
                        self.strain_solver_data[i].action_strain_coefficient = self
                            .get_coefficient_value(
                                action_duration,
                                self.strain_constants.t_jack_lower_boundary_ms,
                                self.strain_constants.t_jack_upper_boundary_ms,
                                self.strain_constants.t_jack_max_strain_value,
                                self.strain_constants.t_jack_curve_exponential,
                            );
                    } else {
                        self.strain_solver_data[i].finger_action = FingerAction::Bracket;
                        self.strain_solver_data[i].action_strain_coefficient = self
                            .get_coefficient_value(
                                action_duration,
                                self.strain_constants.bracket_lower_boundary_ms,
                                self.strain_constants.bracket_upper_boundary_ms,
                                self.strain_constants.bracket_max_strain_value,
                                self.strain_constants.bracket_curve_exponential,
                            );
                    }
                    break;
                }
            }
        }
    }

    fn compute_for_roll_manipulation(&mut self) {
        let mut manipulation_index = 0.;

        for i in 0..self.strain_solver_data.len() {
            let mut manipulation_found = false;

            if self.strain_solver_data[i]
                .next_strain_solver_data_on_current_hand
                .is_some()
                && self.strain_solver_data[self.strain_solver_data[i]
                    .next_strain_solver_data_on_current_hand
                    .unwrap()]
                .next_strain_solver_data_on_current_hand
                .is_some()
            {
                let middle = &self.strain_solver_data[self.strain_solver_data[i]
                    .next_strain_solver_data_on_current_hand
                    .unwrap()];
                let last = &self.strain_solver_data
                    [middle.next_strain_solver_data_on_current_hand.unwrap()];
                if self.strain_solver_data[i].finger_action == FingerAction::Roll
                    && middle.finger_action == FingerAction::Roll
                    && self.strain_solver_data[i].finger_state == last.finger_state
                {
                    let duration_ratio = (self.strain_solver_data[i].finger_action_duration_ms
                        / middle.finger_action_duration_ms)
                        .max(
                            middle.finger_action_duration_ms
                                / self.strain_solver_data[i].finger_action_duration_ms,
                        );

                    if duration_ratio >= self.strain_constants.roll_ratio_tolerance_ms {
                        let duration_multiplier = 1.
                            / (1.
                                + (duration_ratio - 1.)
                                    * self.strain_constants.roll_ratio_multiplier);

                        let manipulation_found_ratio = 1.
                            - manipulation_index / self.strain_constants.roll_max_length
                                * (1. - self.strain_constants.roll_length_multiplier);

                        self.strain_solver_data[i].roll_manipulation_strain_multiplier =
                            duration_multiplier * manipulation_found_ratio;

                        manipulation_found = true;
                        self.roll_inaccuracy_confidence += 1.;

                        if manipulation_index < self.strain_constants.roll_max_length {
                            manipulation_index += 1.;
                        }
                    }
                }
            }

            if !manipulation_found && manipulation_index > 0. {
                manipulation_index -= 1.;
            }
        }
    }

    fn compute_for_jack_manipulation(&mut self) {
        let mut long_jack_size = 0.;

        for i in 0..self.strain_solver_data.len() {
            let mut manipulation_found = false;

            if let Some(next) = self.strain_solver_data[i].next_strain_solver_data_on_current_hand {
                let next = &self.strain_solver_data[next];
                if self.strain_solver_data[i].finger_action == FingerAction::SimpleJack
                    && next.finger_action == FingerAction::SimpleJack
                {
                    let duration_value = (1f32).min(0f32.max(
                        (self.strain_constants.vibro_action_duration_ms
                            + self.strain_constants.vibro_action_tolerance_ms
                            - self.strain_solver_data[i].finger_action_duration_ms)
                            / self.strain_constants.vibro_action_tolerance_ms,
                    ));

                    let duration_multiplier =
                        1. - duration_value * (1. - self.strain_constants.vibro_multiplier);

                    let manipulation_found_ratio = 1.
                        - long_jack_size / self.strain_constants.vibro_max_length
                            * (1. - self.strain_constants.vibro_length_multiplier);

                    self.strain_solver_data[i].roll_manipulation_strain_multiplier =
                        duration_multiplier * manipulation_found_ratio;

                    manipulation_found = true;
                    self.vibro_inaccuracy_confidence += 1.;

                    if long_jack_size < self.strain_constants.vibro_max_length {
                        long_jack_size += 1.;
                    }
                }
            }

            if !manipulation_found {
                long_jack_size = 0.;
            }
        }
    }

    fn compute_for_ln_multiplier(&mut self) {
        for i in 0..self.strain_solver_data.len() {
            if self.strain_solver_data[i].end_time > self.strain_solver_data[i].start_time {
                let duration_value = 1.
                    - 1f32.min(0f32.max(
                        (self.strain_constants.ln_layer_threshold_ms
                            + self.strain_constants.ln_layer_tolerance_ms
                            - (self.strain_solver_data[i].end_time
                                - self.strain_solver_data[i].start_time))
                            / self.strain_constants.ln_layer_tolerance_ms,
                    ));
                let base_multiplier =
                    1. + duration_value * self.strain_constants.ln_base_multiplier;

                for k in self.strain_solver_data[i].hit_objects.iter_mut() {
                    k.ln_strain_multiplier = base_multiplier;
                }

                if let Some(next) =
                    self.strain_solver_data[i].next_strain_solver_data_on_current_hand
                {
                    let next = &self.strain_solver_data[next];
                    if next.start_time
                        < self.strain_solver_data[i].end_time
                            - self.strain_constants.ln_end_threshold_ms
                        && next.start_time
                            >= self.strain_solver_data[i].start_time
                                + self.strain_constants.ln_end_threshold_ms
                    {
                        if next.end_time
                            > self.strain_solver_data[i].end_time
                                + self.strain_constants.ln_end_threshold_ms
                        {
                            for k in self.strain_solver_data[i].hit_objects.iter_mut() {
                                k.ln_layer_type = LnLayerType::OutsideRelease;
                                k.ln_strain_multiplier *=
                                    self.strain_constants.ln_release_after_multiplier;
                            }
                        } else if next.end_time > 0. {
                            for k in self.strain_solver_data[i].hit_objects.iter_mut() {
                                k.ln_layer_type = LnLayerType::InsideRelease;
                                k.ln_strain_multiplier *=
                                    self.strain_constants.ln_release_before_multiplier;
                            }
                        } else {
                            for k in self.strain_solver_data[i].hit_objects.iter_mut() {
                                k.ln_layer_type = LnLayerType::InsideTap;
                                k.ln_strain_multiplier *= self.strain_constants.ln_tap_multiplier;
                            }
                        }
                    }
                }
            }
        }
    }

    fn compute_for_pattern_flags(&mut self) {
        if self.vibro_inaccuracy_confidence / self.strain_solver_data.len() as f32 > 0.10 {
            self.qss_pattern_flags |= QssPatternFlags::SimpleVibro;
        }

        if self.roll_inaccuracy_confidence / self.strain_solver_data.len() as f32 > 0.15 {
            self.qss_pattern_flags |= QssPatternFlags::Rolls;
        }
    }

    fn calculate_overall_difficulty(&mut self) -> f32 {
        let mut calculated_diff: f32;

        for data in self.strain_solver_data.iter_mut() {
            data.calculate_strain_value();
        }

        calculated_diff = self
            .strain_solver_data
            .iter()
            .filter(|s| s.hand == Hand::Left || s.hand == Hand::Right)
            .map(|s| s.total_strain_value)
            .sum::<f32>()
            / self
                .strain_solver_data
                .iter()
                .filter(|s| s.hand == Hand::Left || s.hand == Hand::Right)
                .count() as f32;

        let mut bins: Vec<f32> = Vec::new();
        const BIN_SIZE: i32 = 1000;

        let map_start = self
            .strain_solver_data
            .iter()
            .map(|s| s.start_time)
            .reduce(f32::min)
            .unwrap_or_default();
        let map_end = self
            .strain_solver_data
            .iter()
            .map(|s| s.start_time.max(s.end_time))
            .reduce(f32::max)
            .unwrap_or_default();

        for i in ((map_start * 100.) as i32..(map_end * 100.) as i32)
            .step_by((BIN_SIZE * 100) as usize)
            .map(|x| x as f32 * 0.01)
        {
            let values_in_bin: Vec<_> = self
                .strain_solver_data
                .iter()
                .filter(|s| s.start_time >= i && s.start_time < i + BIN_SIZE as f32)
                .collect();

            let average_rating = if !values_in_bin.is_empty() {
                values_in_bin
                    .iter()
                    .map(|s| s.total_strain_value)
                    .sum::<f32>()
                    / values_in_bin.len() as f32
            } else {
                0.
            };

            bins.push(average_rating);
        }

        if !bins.iter().any(|&strain| strain > 0.) {
            return 0.;
        };

        let cutoff_pos = (bins.len() as f32 * 0.4).floor() as usize;
        let mut sorted: Vec<f32> = bins.clone();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let top_40: Vec<f32> = sorted.into_iter().take(cutoff_pos).collect();

        let easy_rating_cutoff = if !top_40.is_empty() {
            top_40.iter().sum::<f32>() / top_40.len() as f32
        } else {
            0.0
        };

        let continuity = bins
            .iter()
            .filter(|&&strain| strain > 0.)
            .map(|strain| (strain / easy_rating_cutoff).sqrt())
            .sum::<f32>()
            / bins.iter().filter(|&&strain| strain > 0.).count() as f32;

        const MAX_CONTINUITY: f32 = 1.00;
        const AVG_CONTINUITY: f32 = 0.85;
        const MIN_CONTINUITY: f32 = 0.60;

        const MAX_ADJUSTMENT: f32 = 1.05;
        const AVG_ADJUSTMENT: f32 = 1.00;
        const MIN_ADJUSTMENT: f32 = 0.90;

        let continuity_adjustment = if continuity > AVG_CONTINUITY {
            let continuity_factor =
                1. - (continuity - AVG_CONTINUITY) / (MAX_CONTINUITY - AVG_CONTINUITY);
            AVG_ADJUSTMENT.min(
                MIN_ADJUSTMENT
                    .max(continuity_factor * (AVG_ADJUSTMENT - MIN_ADJUSTMENT) + MIN_ADJUSTMENT),
            )
        } else {
            let continuity_factor =
                1.0 - (continuity - MIN_CONTINUITY) / (AVG_CONTINUITY - MIN_CONTINUITY);
            MAX_ADJUSTMENT.min(
                AVG_ADJUSTMENT
                    .max(continuity_factor * (MAX_ADJUSTMENT - AVG_ADJUSTMENT) + AVG_ADJUSTMENT),
            )
        };

        calculated_diff *= continuity_adjustment;

        const MAX_SHORT_MAP_ADJUSTMENT: f32 = 0.75;
        const SHORT_MAP_THRESHOLD: f32 = 60. * 1000.;

        let true_drain_time = bins.len() as f32 * continuity * BIN_SIZE as f32;

        let short_map_adjustment = (0.25 * f32::sqrt(true_drain_time / SHORT_MAP_THRESHOLD) + 0.75)
            .clamp(MAX_SHORT_MAP_ADJUSTMENT, 1.);

        calculated_diff * short_map_adjustment
    }

    fn compute_note_density_data(&mut self, rate: f32) {
        self.average_note_density = 1000. * self.map.hit_objects.len() as f32
            / (self.map.length() as f32 * (-0.5 * rate + 1.5));
    }

    fn get_coefficient_value(
        &self,
        duration: f32,
        x_min: f32,
        x_max: f32,
        strain_max: f32,
        exp: f32,
    ) -> f32 {
        const LOWEST_DIFFICULTY: f32 = 1.0;
        const DENSITY_MULTIPLIER: f32 = 0.266;
        const DENSITY_DIFFICULTY_MIN: f32 = 0.4;

        let ratio = f32::max(0., 1. - (duration - x_min) / (x_max - x_min));

        if ratio == 0. && self.average_note_density < 4. {
            if self.average_note_density < 1. {
                return DENSITY_DIFFICULTY_MIN;
            }

            return self.average_note_density * DENSITY_MULTIPLIER + 0.134;
        }

        LOWEST_DIFFICULTY + (strain_max - LOWEST_DIFFICULTY) * f32::powf(ratio, exp)
    }
}
