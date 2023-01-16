#[derive(Default, Clone, Copy, Eq, PartialEq, Debug)]
pub enum Hand {
    Left,
    #[default]
    Right,
    Ambiguous,
}
