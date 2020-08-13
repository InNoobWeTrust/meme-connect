#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        match (*self, other) {
            (Direction::UP, Direction::DOWN) | (Direction::DOWN, Direction::UP) => true,
            (Direction::LEFT, Direction::RIGHT) | (Direction::RIGHT, Direction::LEFT) => true,
            _ => false,
        }
    }
}
