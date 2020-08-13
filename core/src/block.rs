use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub x: usize,
    pub y: usize,
}

impl Block {
    /// Generate neighbour blocks
    pub fn neighbours(&self) -> Vec<(Direction, Block)> {
        (vec![
            (Direction::UP, (self.x as isize, self.y as isize - 1)),
            (Direction::DOWN, (self.x as isize, self.y as isize + 1)),
            (Direction::LEFT, (self.x as isize - 1, self.y as isize)),
            (Direction::RIGHT, (self.x as isize + 1, self.y as isize)),
        ])
        .iter()
        .filter(|&(_, (x, y))| *x >= 0 && *y >= 0)
        .map(|&(direction, (x, y))| {
            (
                direction,
                Block {
                    x: x as usize,
                    y: y as usize,
                },
            )
        })
        .collect()
    }

    pub fn distance_sqr(&self, other: &Self) -> i64 {
        (self.x as i64 - other.x as i64).pow(2) + (self.x as i64 - other.x as i64).pow(2)
    }
}
