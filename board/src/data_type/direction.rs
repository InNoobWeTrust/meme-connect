#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Axis {
    Column,
    Row,
}

impl Direction {
    pub fn is_opposite(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Up, &Self::Down) | (&Self::Down, &Self::Up) => true,
            (&Self::Left, &Self::Right) | (&Self::Right, &Self::Left) => true,
            _ => false,
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Self::Up | Self::Down => Axis::Column,
            Self::Left | Self::Right => Axis::Row,
        }
    }
}
