use crate::data_type::traits::*;
use crate::prelude::*;
use std::{iter, ops::RangeInclusive};

#[derive(Clone)]
pub struct Path {
    /// The origin of travel path
    origin: Cell,
    /// Direction of travel
    direction: Direction,
    /// Number of free cells on the path
    free_cells: usize,
}

impl Path {
    /// Reflect path on its axis, yielded column/row of free cells.
    /// The direction of path is not guaranteed.
    fn axis_reflection(&self) -> RangeInclusive<usize> {
        match self.direction {
            Direction::Up => ((self.origin.row - self.free_cells)..=(self.origin.row - 1)),
            Direction::Down => ((self.origin.row + 1)..=(self.origin.row + self.free_cells)),
            Direction::Left => ((self.origin.column - self.free_cells)..=(self.origin.column - 1)),
            Direction::Right => ((self.origin.column + 1)..=(self.origin.column + self.free_cells)),
        }
    }

    /// Get the steps.
    pub fn steps(&self) -> impl Iterator<Item = Cell> {
        self.origin.walk(self.direction).take(self.free_cells)
    }

    /// Check if 2 paths is on same line
    fn same_line(&self, other: &Self) -> bool {
        match (
            self.direction.axis(),
            other.direction.axis(),
            self.origin.column == other.origin.column,
            self.origin.row == other.origin.row,
        ) {
            (_, _, true, true) => true,
            (Axis::Column, Axis::Column, true, _) => true,
            (Axis::Row, Axis::Row, _, true) => true,
            _ => false,
        }
    }

    /// Check if overlap.
    fn is_overlap(&self, other: &Self) -> bool {
        // If one's obstacle is another's origin, it is overlap
        self.origin
            .walk(self.direction)
            .take(self.free_cells + 1)
            .last()
            .unwrap()
            == other.origin
    }

    /// Find crossed cell if exist, return the cell
    fn find_crossed(&self, other: &Self) -> Option<Cell> {
        // Check if cross point exist
        match (self.direction.axis(), other.direction.axis()) {
            (Axis::Column, Axis::Row) => Some(Cell {
                column: self.origin.column,
                row: other.origin.row,
            }),
            (Axis::Row, Axis::Column) => Some(Cell {
                column: other.origin.column,
                row: self.origin.row,
            }),
            _ => None,
        }
        .and_then(|cross_point| {
            if self.steps().any(|cell| cell == cross_point)
                && other.steps().any(|cell| cell == cross_point)
            {
                Some(cross_point)
            } else {
                None
            }
        })
    }

    /// Find the column/row which can bridge the 2 paths.
    fn find_bridge(
        &self,
        other: &Self,
        check_free_step: impl Fn(&Cell) -> bool,
    ) -> Option<(Cell, Cell)> {
        // Only if same axis and not on same line
        if self.direction.axis() != other.direction.axis() && !self.same_line(other) {
            return None;
        }

        let axis = self.direction.axis();
        // Find free bridge between paths
        self.steps()
            .filter_map(|start| {
                match (
                    axis,
                    other.axis_reflection().contains(&start.row),
                    other.axis_reflection().contains(&start.column),
                ) {
                    (Axis::Column, true, _) => Some((
                        start,
                        Cell {
                            column: other.origin.column,
                            row: start.row,
                        },
                    )),
                    (Axis::Row, _, true) => Some((
                        start,
                        Cell {
                            column: start.column,
                            row: other.origin.row,
                        },
                    )),
                    _ => None,
                }
            })
            .find(|(start, stop)| {
                let bridge_direction = start.direction_to(stop).unwrap();
                start
                    .walk(bridge_direction)
                    .take_while(|cell| cell != stop)
                    .all(|cell| check_free_step(&cell))
            })
    }
}

struct RayCast {
    /// The origin where rays are casted
    origin: Cell,
    /// Paths
    casts: Vec<Path>,
}

macro_rules! pair_paths {
    ($first: ident, $second: ident) => {
        $first
            .casts
            .iter()
            .flat_map(|path| iter::repeat(path).zip($second.casts.iter()))
    };
}

impl RayCast {
    fn find_overlap(&self, other: &Self) -> Option<Vec<Cell>> {
        pair_paths!(self, other)
            .find(|(path, other_path)| path.is_overlap(other_path))
            .map(|_| vec![self.origin, other.origin])
    }

    fn find_crossed(&self, other: &Self) -> Option<Vec<Cell>> {
        pair_paths!(self, other)
            .find_map(|(path, other_path)| path.find_crossed(other_path))
            .map(|cross_cell| vec![self.origin, cross_cell, other.origin])
    }

    fn find_bridge(
        &self,
        other: &Self,
        check_free_step: impl Fn(&Cell) -> bool,
    ) -> Option<Vec<Cell>> {
        pair_paths!(self, other)
            .filter_map(|(path, other_path)| {
                path.find_bridge(other_path, |cell| check_free_step(cell))
            })
            .min_by_key(|(first_pole, second_pole)| {
                // Take the bridge with minimum travel path
                ((self.origin.distance_sqr(&first_pole) as f32).sqrt()
                    + (first_pole.distance_sqr(&second_pole) as f32).sqrt()
                    + (second_pole.distance_sqr(&other.origin) as f32).sqrt())
                    as usize
            })
            .map(|(first_pole, second_pole)| {
                vec![self.origin, first_pole, second_pole, other.origin]
            })
    }
}

impl Mapping {
    fn ray_cast(&self, origin: &Cell) -> RayCast {
        let casts = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .map(|direction| Path {
            origin: *origin,
            direction: *direction,
            free_cells: origin
                .walk(*direction)
                .take_while(|cell| {
                    self.check_valid_cell(cell) && self.get_sprite(cell) == NO_SPRITE
                })
                .count(),
        })
        .collect();

        RayCast {
            origin: *origin,
            casts,
        }
    }

    pub fn connect(
        &self,
        first_cell: &Cell,
        second_cell: &Cell,
    ) -> Result<Vec<Cell>, (&'static str, Vec<Path>)> {
        let first_trace = self.ray_cast(first_cell);
        let second_trace = self.ray_cast(second_cell);

        if let Some(connection) = first_trace.find_overlap(&second_trace) {
            println!("Overlap connect");
            return Ok(connection);
        } else if let Some(connection) = first_trace.find_crossed(&second_trace) {
            println!("Crossed connect");
            return Ok(connection);
        } else if let Some(connection) = first_trace.find_bridge(&second_trace, |cell| {
            self.check_valid_cell(cell) && self.get_sprite(cell) == NO_SPRITE
        }) {
            println!("Bridge connect");
            return Ok(connection);
        }

        let paths = [&first_trace.casts[..], &second_trace.casts[..]].concat();
        Err(("Cannot connect", paths))
    }

    pub fn no_more_move(&self) -> bool {
        /// TODO: Integrate raytracing
        false
    }
}
