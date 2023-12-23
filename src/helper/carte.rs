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

    pub fn move_to_clamped(
        self,
        direction: Direction,
        width: usize,
        height: usize,
    ) -> Option<Self> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => {
                let down = self.down();
                (down.y != height).then_some(down)
            }
            Direction::Right => {
                let right = self.right();
                (right.x != width).then_some(right)
            }
            Direction::Left => self.left(),
        }
    }

    pub fn neighbors_clamped(self, width: usize, height: usize) -> [Option<Self>; 4] {
        [
            self.up(),
            {
                let down = self.down();
                (down.y != height).then_some(down)
            },
            self.left(),
            {
                let right = self.right();
                (right.x != width).then_some(right)
            },
        ]
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IPos {
    pub x: isize,
    pub y: isize,
}

impl IPos {
    pub fn wrapped(self, width: usize, height: usize) -> Pos {
        let width = isize::try_from(width).unwrap();
        let height = isize::try_from(height).unwrap();

        let x = self.x % width;
        let y = self.y % height;

        let x = if x < 0 { x + width } else { x };
        let y = if y < 0 { y + height } else { y };

        Pos {
            x: usize::try_from(x).unwrap(),
            y: usize::try_from(y).unwrap(),
        }
    }

    pub fn neighbors(self) -> [Self; 4] {
        let Self { x, y } = self;
        [
            {
                let y = y - 1;
                Self { x, y }
            },
            {
                let y = y + 1;
                Self { x, y }
            },
            {
                let x = x - 1;
                Self { x, y }
            },
            {
                let x = x + 1;
                Self { x, y }
            },
        ]
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

    #[inline]
    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[inline]
    pub fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

/* -------------------------------------------------------------------------- */
