#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Loc<T> {
    pub x: T,
    pub y: T,
}
