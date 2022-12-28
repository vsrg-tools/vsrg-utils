#[derive(Default, Clone, Copy, Debug)]
pub enum LnLayerType {
    #[default]
    None,
    InsideRelease,
    OutsideRelease,
    InsideTap,
}
