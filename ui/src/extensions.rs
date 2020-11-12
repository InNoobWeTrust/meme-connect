use crate::region::*;
use ::board::data_type::cell::Cell;

impl Size {
    /// Fit game board in screen
    pub fn game_board_region(&self, columns: usize, rows: usize) -> Region {
        // TODO: Layout game board based on screen orientation
        let x = self.width * 0.1;
        let y = self.height * 0.1;
        let board_coord = Coordinate { x, y };

        let width = self.width * 0.8;
        let height = self.height * 0.8;
        let board_size = Size { width, height };

        // TODO: Keep uniform texture ratio
        let cell_width = width / (columns + 2) as f32; // Border columns
        let cell_height = height / (rows + 2) as f32; // Border rows
        let cell_size = Size {
            width: cell_width,
            height: cell_height,
        };

        Region {
            coord: board_coord,
            size: board_size,
            meta: Meta::BOARD(cell_size),
        }
    }
}

impl Region {
    pub fn cell_region(&self, cell: &Cell) -> Result<Region, &'static str> {
        if let Meta::BOARD(cell_size) = self.meta {
            let x = self.coord.x + (cell.column as f32 - 1.0) * cell_size.width;
            let y = self.coord.y + (cell.row as f32 - 1.0) * cell_size.height;
            let coord = Coordinate { x, y };
            Ok(Region {
                coord,
                size: cell_size,
                meta: Meta::CELL(cell.column, cell.row),
            })
        } else {
            Err("Expect board region, found generic region")
        }
    }

    /// Map from coordinate inside region to board column and row
    pub fn cell_from_coord(&self, coord: &Coordinate) -> Result<Cell, &'static str> {
        if let Meta::BOARD(cell_size) = self.meta {
            if !self.contain_coord(&coord) {
                return Err("Coord outside board region");
            }
            // board has invisible border, so cell's column and row starts at 1
            let column: usize = ((coord.x - self.coord.x) / cell_size.width).ceil() as usize;
            let row: usize = ((coord.y - self.coord.y) / cell_size.height).ceil() as usize;
            Ok(Cell { column, row })
        } else {
            Err("Expect board region, found generic region")
        }
    }
}
