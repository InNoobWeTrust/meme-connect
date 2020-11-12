use super::direction::Direction;
use std::{collections::HashMap, iter};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell {
    pub column: usize,
    pub row: usize,
}

impl Cell {
    pub fn direction_to(&self, other: &Self) -> Option<Direction> {
        match (self.column == other.column, self.row == other.row) {
            (true, false) => {
                if self.row < other.row {
                    Some(Direction::Down)
                } else {
                    Some(Direction::Up)
                }
            }
            (false, true) => {
                if self.column < other.column {
                    Some(Direction::Right)
                } else {
                    Some(Direction::Left)
                }
            }
            _ => None,
        }
    }

    pub fn neighbour(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Cell {
                column: self.column,
                row: (self.row as isize - 1) as usize,
            },
            Direction::Down => Cell {
                column: self.column,
                row: (self.row as isize + 1) as usize,
            },
            Direction::Left => Cell {
                column: (self.column as isize - 1) as usize,
                row: self.row,
            },
            Direction::Right => Cell {
                column: (self.column as isize + 1) as usize,
                row: self.row,
            },
        }
    }

    /// Generate neighbour blocks
    pub fn neighbours(&self) -> HashMap<Direction, Self> {
        [
            (Direction::Up, (self.column as isize, self.row as isize - 1)),
            (
                Direction::Down,
                (self.column as isize, self.row as isize + 1),
            ),
            (
                Direction::Left,
                (self.column as isize - 1, self.row as isize),
            ),
            (
                Direction::Right,
                (self.column as isize + 1, self.row as isize),
            ),
        ]
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

    pub fn is_neighbour(&self, other: &Self) -> bool {
        self.neighbours().iter().any(|(_, cell)| cell == other)
    }

    /// Walk on direction. Warning: must chain call with stop condition or it
    /// will be endless iteration.
    pub fn walk(&self, direction: Direction) -> impl Iterator<Item = Self> {
        iter::successors(Some(self.neighbour(&direction)), move |cell| {
            Some(cell.neighbour(&direction))
        })
    }

    /// Assume board is small enough so type conversion doesn't truncate value
    pub fn distance_sqr(&self, other: &Self) -> isize {
        (self.column as isize - other.column as isize).pow(2)
            + (self.column as isize - other.column as isize).pow(2)
    }
}
