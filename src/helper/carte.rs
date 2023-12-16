/* -------------------------------------------------------------------------- */

/// 0-based 2d position. bottom is positive Y, right is positive X.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn up(self) -> Option<Self> {
        let Self { x, y } = self;
        let y = y.checked_sub(1)?;
        Some(Self { x, y })
    }

    pub fn down(self) -> Self {
        let Self { x, y } = self;
        let y = y + 1;
        Self { x, y }
    }

    pub fn left(self) -> Option<Self> {
        let Self { x, y } = self;
        let x = x.checked_sub(1)?;
        Some(Self { x, y })
    }

    pub fn right(self) -> Self {
        let Self { x, y } = self;
        let x = x + 1;
        Self { x, y }
    }

    pub fn move_to(self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => Some(self.down()),
            Direction::Right => Some(self.right()),
            Direction::Left => self.left(),
        }
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    #[inline]
    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

/* -------------------------------------------------------------------------- */
