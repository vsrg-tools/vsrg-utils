use super::{FingerState, LnLayerType};
use crate::maps::structures::HitObjectInfo;

#[derive(Default, Clone, Copy, Debug)]
pub struct StrainSolverHitObject {
    pub hit_object: HitObjectInfo,
    pub finger_state: FingerState,
    pub ln_layer_type: LnLayerType,
    pub ln_strain_multiplier: f32,
    pub strain_value: f32,
}

impl StrainSolverHitObject {
    pub fn new(hit_ob: HitObjectInfo) -> Self {
        Self {
            hit_object: hit_ob,
            ln_strain_multiplier: 1.,
            ..Default::default()
        }
    }
}
