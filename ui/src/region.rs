use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Meta {
    /// TODO: Screen should keep info on orientation
    SCREEN,
    /// Board with specific cell size
    BOARD(Size),
    /// Cell at specific column and row
    CELL(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Region {
    pub coord: Coordinate,
    pub size: Size,
    pub meta: Meta,
}

pub struct RegionContainer {
    pub own: Region,
    pub childrens: Vec<RegionContainer>,
}

impl Region {
    /// Fit the render region to expected aspect ratio
    pub fn aspect_fit(&self, aspect_ratio: &(f32, f32)) -> Result<Region, &'static str> {
        let width_from_height_by_aspect = self.size.height * aspect_ratio.0 / aspect_ratio.1;
        let height_from_width_by_aspect = self.size.width * aspect_ratio.1 / aspect_ratio.0;
        match (
            self.size.width.partial_cmp(&width_from_height_by_aspect),
            self.size.height.partial_cmp(&height_from_width_by_aspect),
        ) {
            (Some(Ordering::Less), _) => Ok(Region {
                coord: Coordinate {
                    x: self.coord.x + (self.size.width - width_from_height_by_aspect) / 2.,
                    y: self.coord.y,
                },
                size: Size {
                    width: width_from_height_by_aspect,
                    height: self.size.height,
                },
                meta: self.meta,
            }),
            (_, Some(Ordering::Less)) => Ok(Region {
                coord: Coordinate {
                    x: self.coord.x,
                    y: self.coord.y + (self.size.height - height_from_width_by_aspect) / 2.,
                },
                size: Size {
                    width: self.size.width,
                    height: height_from_width_by_aspect,
                },
                meta: self.meta,
            }),
            _ => Err("Error calculating render region"),
        }
    }

    /// Check if coordinate is inside the region
    pub fn contain_coord(&self, coord: &Coordinate) -> bool {
        // Check x in range
        if coord.x < self.coord.x {
            return false;
        }
        if coord.x > self.coord.x + self.size.width {
            return false;
        }

        // Check y in range
        if coord.y < self.coord.y {
            return false;
        }
        if coord.y > self.coord.y + self.size.height {
            return false;
        }

        true
    }

    pub fn center(&self) -> Coordinate {
        Coordinate {
            x: self.coord.x + self.size.width / 2.,
            y: self.coord.y + self.size.height / 2.,
        }
    }
}
