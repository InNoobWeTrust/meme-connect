#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub x: usize,
    pub y: usize,
}

impl Block {
    pub fn new(x: usize, y: usize) -> Block {
        Block { x, y }
    }
}
