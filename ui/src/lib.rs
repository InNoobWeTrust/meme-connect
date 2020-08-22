#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegionSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Region {
    pub coord: Coordinate,
    pub size: RegionSize,
}

pub struct GameBoard {
    pub board: Region,
    pub block_size: RegionSize,
}

impl GameBoard {
    pub fn new(screen_size: RegionSize, cols: usize, rows: usize) -> Self {
        // TODO: Layout game board based on screen orientation
        let x = screen_size.width * 0.1;
        let y = screen_size.height * 0.1;
        let board_coord = Coordinate { x, y };

        let width = screen_size.width * 0.8;
        let height = screen_size.height * 0.8;
        let board_size = RegionSize { width, height };

        let board = Region {
            coord: board_coord,
            size: board_size,
        };

        // TODO: Keep uniform texture ratio
        let block_width = width / cols as f32;
        let block_height = height / rows as f32;
        let block_size = RegionSize {
            width: block_width,
            height: block_height,
        };

        Self { board, block_size }
    }

    // TODO: Auto-fit instead of stretching
    pub fn calc_block_region(&self, col: usize, row: usize) -> Region {
        let x = self.board.coord.x + col as f32 * self.block_size.width;
        let y = self.board.coord.y + row as f32 * self.block_size.height;
        let coord = Coordinate { x, y };
        Region {
            coord,
            size: self.block_size,
        }
    }
}
