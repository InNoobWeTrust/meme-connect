#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn is_opposite(&self, other: Self) -> bool {
        match (*self, other) {
            (Self::UP, Self::DOWN) | (Self::DOWN, Self::UP) => true,
            (Self::LEFT, Self::RIGHT) | (Self::RIGHT, Self::LEFT) => true,
            _ => false,
        }
    }
}
