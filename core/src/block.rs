use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub column: usize,
    pub row: usize,
}

impl Block {
    /// Generate neighbour blocks
    pub fn neighbours(&self) -> Vec<(Direction, Self)> {
        (vec![
            (Direction::UP, (self.column as isize, self.row as isize - 1)),
            (
                Direction::DOWN,
                (self.column as isize, self.row as isize + 1),
            ),
            (
                Direction::LEFT,
                (self.column as isize - 1, self.row as isize),
            ),
            (
                Direction::RIGHT,
                (self.column as isize + 1, self.row as isize),
            ),
        ])
        .iter()
        .filter(|&(_, (x, y))| *x >= 0 && *y >= 0)
        .map(|&(direction, (x, y))| {
            (
                direction,
                Self {
                    column: x as usize,
                    row: y as usize,
                },
            )
        })
        .collect()
    }

    pub fn distance_sqr(&self, other: &Self) -> i64 {
        (self.column as i64 - other.column as i64).pow(2)
            + (self.column as i64 - other.column as i64).pow(2)
    }
}
