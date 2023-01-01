#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Hand {
    Left,
    #[default]
    Right,
    Ambiguous,
}
