use super::ReplayKeyPressState;

#[derive(Debug, Copy, Clone)]
pub struct ReplayFrame {
    pub time: i32,
    pub keys: ReplayKeyPressState,
}
