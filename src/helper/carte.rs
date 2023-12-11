/// 0-based 2d position. bottom is positive Y, right is positive X.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}
