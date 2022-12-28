#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum FingerAction {
    #[default]
    None,
    SimpleJack,
    TechnicalJack,
    Roll,
    Bracket,
}
