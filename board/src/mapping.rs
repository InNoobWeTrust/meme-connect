use crate::prelude::*;
use std::iter::repeat;

pub struct Mapping {
    pub columns: usize,
    pub rows: usize,
    data: Vec<SpriteId>,
}

impl Default for Mapping {
    fn default() -> Self {
        Self {
            columns: 12,
            rows: 12,
            data: vec![NO_SPRITE; 12 * 12],
        }
    }
}

impl Mapping {
    /// Max size is limited to 256x256
    pub fn new(columns: u8, rows: u8) -> Self {
        let actual_columns = columns as usize + 2;
        let actual_rows = rows as usize + 2;
        Self {
            columns: actual_columns,
            rows: actual_rows,
            data: vec![NO_SPRITE; actual_columns * actual_rows],
        }
    }

    fn cell2index(&self, cell: &Cell) -> usize {
        cell.row * self.columns + cell.column
    }

    ///////////////////////////////////////////////////////////////////////////
    ////////////////////////////// View of data ///////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_sprite(&self, cell: &Cell) -> SpriteId {
        self.data[cell.row * self.columns + cell.column]
    }

    fn get_row(&self, index: usize) -> Vec<SpriteId> {
        self.data
            .iter()
            .skip(index * self.columns)
            .take(self.columns)
            .copied()
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<SpriteId>> {
        (0..self.rows).map(|y| self.get_row(y)).collect::<Vec<_>>()
    }

    pub fn mutable_cells(&self) -> impl Iterator<Item = Cell> {
        let columns = self.columns;
        let rows = self.rows;
        (1..=columns - 2)
            .flat_map(move |column| repeat(column).zip(1..=rows - 2))
            .map(|(column, row)| Cell { column, row })
    }

    pub fn _fmt(&self) -> String {
        self.get_rows()
            .iter()
            .map(|line| {
                format!(
                    "|{}|",
                    line.iter()
                        .map(|sprite| format!("{:^5}", sprite))
                        .collect::<Vec<_>>()
                        .join("|")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    ///////////////////////////////////////////////////////////////////////////
    /////////////////////////// Position validation ///////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    pub fn check_valid_cell(&self, cell: &Cell) -> bool {
        cell.column < self.columns && cell.row < self.rows
    }

    fn check_if_border(&self, cell: &Cell) -> bool {
        cell.column == 0                           // Left border
            || cell.column == self.columns - 1     // Right border
            || cell.row == 0                       // Top border
            || cell.row == self.rows - 1 // Bottom border
    }

    pub fn check_fillable_cell(&self, cell: &Cell) -> Result<(), &'static str> {
        if self.check_if_border(cell) {
            return Err("cell is at border");
        }
        match self.data[self.cell2index(cell)] {
            NO_SPRITE => Ok(()),
            _ => Err("cell is occupied"),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////// Manipulate data /////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    pub fn clear_cell(&mut self, cell: &Cell) {
        let idx = self.cell2index(cell);
        self.data[idx] = NO_SPRITE;
    }

    pub fn fill_cell(&mut self, cell: &Cell, sprite: SpriteId) -> Result<(), &'static str> {
        self.check_fillable_cell(cell)?;
        let idx = self.cell2index(cell);
        self.data[idx] = sprite;
        Ok(())
    }

    /// Set pair of sprites in order, the position is as instructed by `regions`.
    /// Note: `regions` is consumed
    pub fn fill_regions(
        &mut self,
        regions: &mut Vec<Cell>,
        sprite_list: impl Iterator<Item = SpriteId>,
    ) -> Result<(), &'static str> {
        for sprite in sprite_list {
            if regions.len() < 2 {
                break;
            }
            for _ in 0..2 {
                let cell = regions.pop().unwrap();
                self.fill_cell(&cell, sprite)?;
            }
        }
        Ok(())
    }
}
