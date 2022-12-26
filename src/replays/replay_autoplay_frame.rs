use super::ReplayKeyPressState;
use crate::maps::structures::HitObjectInfo;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct ReplayAutoplayFrame {
    pub type_: ReplayAutoplayFrameType,
    pub time: i32,
    pub keys: ReplayKeyPressState,
    pub hit_object: HitObjectInfo,
}

#[derive(Debug, Copy, Clone)]
pub enum ReplayAutoplayFrameType {
    Press,
    Release,
}
