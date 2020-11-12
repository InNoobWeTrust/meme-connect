use crate::prelude::*;

pub struct Couple {
    /// Still need to render removed cells on destroy
    pub remnants: [(Cell, SpriteId); 2],
    /// Nodes for connection
    pub nodes: Vec<Cell>,
    /// Time added
    pub epoch: f64,
}

macro_rules! define_buf {
    ($st: ident, $t: ty, $size: expr) => {
        #[derive(Default)]
        struct $st {
            idx: usize,
            buf: [Option<$t>; $size],
        }

        impl $st {
            /// Auto discard old value
            fn push(&mut self, item: $t) {
                self.buf[self.idx] = Some(item);
                self.idx = (self.idx + 1) % self.buf.len();
            }

            fn last_idx(&self) -> usize {
                (self.idx + self.buf.len() - 1) % self.buf.len()
            }

            fn alternate(&mut self, item: $t) {
                let last_idx = self.last_idx();
                self.buf[last_idx] = Some(item);
            }

            fn latest(&self) -> Option<&$t> {
                let last_idx = self.last_idx();
                self.buf[last_idx].as_ref()
            }

            /// Reset all values in buffer to None
            fn clear(&mut self) {
                (0..self.buf.len()).for_each(|i| {
                    self.buf[i] = None;
                });
                self.idx = 0;
            }

            fn poll(&self) -> &[Option<$t>] {
                &self.buf
            }
        }
    };
}

define_buf!(SelectBuf, Cell, 2);
define_buf!(DestroyBuf, Couple, 3);

#[derive(Default)]
pub struct CellConnector {
    /// Buffer for selected pair
    select_buf: SelectBuf,
    /// Buffer for destroying pairs
    destroy_buf: DestroyBuf,
}

impl CellConnector {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn select(&mut self, cell: Cell) {
        if let Some(&prev) = self.select_buf.latest() {
            // Deselect case
            if prev == cell {
                self.select_buf.clear();
                return;
            }
        }
        self.select_buf.push(cell);
    }

    pub fn alter_selection(&mut self, cell: Cell) {
        self.select_buf.alternate(cell);
    }

    /// Update selection and send matching couple to destroy buffer
    pub fn update(&mut self, mapping: &mut Mapping, instant: f64) -> Result<(), Vec<Path>> {
        let selection: Vec<&Cell> = self
            .select_buf
            .poll()
            .iter()
            .filter_map(|o| o.as_ref())
            .collect();

        if selection.len() < 2 {
            return Ok(());
        }

        // Clear selection if not the same sprite
        if mapping.get_sprite(selection[0]) != mapping.get_sprite(selection[1]) {
            self.select_buf.clear();
            return Ok(());
        }

        // Connecting
        let result = match mapping.connect(selection[0], selection[1]).map(|nodes| {
            let remnants = [
                (*selection[0], mapping.get_sprite(selection[0])),
                (*selection[1], mapping.get_sprite(selection[1])),
            ];
            Couple {
                remnants,
                nodes,
                epoch: instant,
            }
        }) {
            Ok(couple) => {
                // Remove couple from mapping
                mapping.clear_cell(selection[0]);
                mapping.clear_cell(selection[1]);

                // Add couple to destroy buffer
                self.destroy_buf.push(couple);
                Ok(())
            }
            Err((e, conquered)) => {
                dbg!(e);
                Err(conquered)
            }
        };
        // clear selection after match
        self.select_buf.clear();

        result
    }

    pub fn get_selection(&self) -> Option<&Cell> {
        self.select_buf.latest()
    }

    pub fn poll_destroying(&self) -> Vec<&Couple> {
        self.destroy_buf
            .poll()
            .iter()
            .filter_map(|o| o.as_ref())
            .collect()
    }
}
