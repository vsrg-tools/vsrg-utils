use std::{cell::RefCell, rc::Rc};

use super::{FingerAction, FingerState, Hand, StrainSolverHitObject};

#[derive(Default, Clone, Debug)]
pub struct StrainSolverData {
    pub hit_objects: Vec<StrainSolverHitObject>,
    pub next_strain_solver_data_on_current_hand: Option<Rc<RefCell<StrainSolverData>>>,
    pub start_time: f32,
    pub end_time: f32,
    pub action_strain_coefficient: f32,
    pub pattern_strain_multiplier: f32,
    pub roll_manipulation_strain_multiplier: f32,
    pub jack_manipulation_strain_multiplier: f32,
    pub total_strain_value: f32,
    pub hand: Hand,
    pub finger_action: FingerAction,
    pub finger_action_duration_ms: f32,
    pub pattern: String,
    pub hand_chord: bool,
    pub finger_state: FingerState,
}

impl StrainSolverData {
    pub fn new(hit_ob: StrainSolverHitObject, rate: Option<f32>) -> Self {
        let mut self_ = Self {
            action_strain_coefficient: 1.,
            pattern_strain_multiplier: 1.1,
            roll_manipulation_strain_multiplier: 1.,
            jack_manipulation_strain_multiplier: 1.,
            ..Default::default()
        };

        self_.start_time = hit_ob.hit_object.start_time as f32 / rate.unwrap_or(1.);
        self_.end_time = hit_ob.hit_object.end_time as f32 / rate.unwrap_or(1.);
        self_.hit_objects.push(hit_ob);

        self_
    }

    pub fn get_next(&self) -> &Rc<RefCell<Self>> {
        self.next_strain_solver_data_on_current_hand
            .as_ref()
            .unwrap()
    }

    pub fn calculate_strain_value(&mut self) {
        for mut hit_ob in self.hit_objects.iter_mut() {
            hit_ob.strain_value = self.action_strain_coefficient
                * self.pattern_strain_multiplier
                * self.roll_manipulation_strain_multiplier
                * self.jack_manipulation_strain_multiplier
                * hit_ob.ln_strain_multiplier;
            self.total_strain_value += hit_ob.strain_value
        }

        self.total_strain_value /= self.hit_objects.len() as f32;
    }

    pub fn solve_finger_state(&mut self) {
        for hit_ob in self.hit_objects.iter_mut() {
            self.finger_state |= hit_ob.finger_state;
        }
    }
}
