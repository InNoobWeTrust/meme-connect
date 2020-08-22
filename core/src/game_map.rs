use crate::{block::Block, matcher::Matcher, meme::*, shadow::ShadowBlend, track::*};

pub struct GameMap {
    pub columns: usize,
    pub rows: usize,
    data: Vec<Meme>,
}

impl GameMap {
    pub fn new(columns: u8, rows: u8) -> Option<Self> {
        if 0 != columns && 0 != rows && 0 == ((columns - 2) * (rows - 2)) % 2 {
            Some(Self {
                columns: columns as usize,
                rows: rows as usize,
                data: vec![NO_MEME; columns as usize * rows as usize],
            })
        } else {
            None
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    ////////////////////////////// View of data ///////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    pub fn cell(&self, blk: &Block) -> Meme {
        self.data[blk.row * self.columns + blk.column]
    }

    fn col(&self, index: usize) -> Vec<Meme> {
        self.data
            .iter()
            .skip(index)
            .step_by(self.columns)
            .copied()
            .collect::<Vec<_>>()
    }

    fn row(&self, index: usize) -> Vec<Meme> {
        self.data
            .iter()
            .skip(index * self.columns)
            .take(self.columns)
            .copied()
            .collect::<Vec<_>>()
    }

    fn rows(&self) -> Vec<Vec<Meme>> {
        (0..self.rows).map(|y| self.row(y)).collect::<Vec<_>>()
    }

    fn cols(&self) -> Vec<Vec<Meme>> {
        (0..self.columns).map(|x| self.col(x)).collect::<Vec<_>>()
    }

    pub fn playground_blocks(&self) -> Vec<Block> {
        (1..self.columns - 1)
            .flat_map(move |x| (1..self.rows - 1).map(move |y| Block { column: x, row: y }))
            .collect::<Vec<_>>()
    }

    pub fn _fmt(&self) -> String {
        self.rows()
            .iter()
            .map(|line| {
                format!(
                    "|{}|",
                    line.iter()
                        .map(|meme| format!("{:^5}", meme))
                        .collect::<Vec<_>>()
                        .join("|")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    ///////////////////////////////////////////////////////////////////////////
    ////////////////////////// Position validation ////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    fn is_idx_border(&self, idx: usize) -> bool {
        // Top border
        idx < self.columns ||
            // Bottom border
            idx > self.columns * self.rows - 1 ||
            // Left border
            idx % self.columns == 0 ||
            // Right border
            idx % self.columns == self.columns - 1
    }

    fn block2idx(&self, blk: &Block) -> usize {
        blk.row * self.columns + blk.column
    }

    fn check_border_block(&self, blk: &Block) -> Result<(), String> {
        if self.is_idx_border(self.block2idx(blk)) {
            Err("border".to_string())
        } else {
            Ok(())
        }
    }

    pub fn check_valid_empty_block(&self, blk: &Block) -> Result<(), String> {
        if let Err(err) = self.check_border_block(blk) {
            return Err(format!("{:?} is at {}", blk, err));
        }
        match self.data[self.block2idx(blk)] {
            NO_MEME => Ok(()),
            meme => Err(format!("{:?} occupied with {}", blk, meme)),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //////////////////////// Helpers to get set meme //////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    pub fn collect_empty_blocks<'a, T>(&self, from: &mut T) -> Vec<&'a Block>
    where
        T: ExactSizeIterator<Item = &'a Block>,
    {
        from.filter(|blk| self.check_valid_empty_block(blk).is_ok())
            .collect::<Vec<_>>()
    }

    pub fn set_meme(&mut self, meme: Meme, blk: &Block) -> Result<(), String> {
        self.check_valid_empty_block(blk)?;
        let idx = self.block2idx(blk);
        self.data[idx] = meme;
        Ok(())
    }

    /// Set pair of memes in order, the position is as instructed by `regions`.
    /// Note: `regions` is consumed
    pub fn set_meme_regions<T>(
        &mut self,
        meme_lst: &mut T,
        regions: &mut Vec<&Block>,
    ) -> Result<(), String>
    where
        T: Iterator<Item = Meme>,
    {
        for meme in meme_lst {
            if 2 > regions.len() {
                break;
            }
            for _ in 0..2 {
                let blk = regions.pop().unwrap();
                if let Err(err) = self.set_meme(meme, blk) {
                    return Err(format!("Cannot set meme at {:?}: {}", blk, err));
                }
            }
        }
        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////
    //////////////////////////// Matching logic ///////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    pub fn cast_horizontal_shadows(
        &self,
        wall_idx: usize,
        cast_ranges: (Option<usize>, Option<usize>),
    ) -> Vec<ShadowBlend> {
        ShadowBlend::pack_from(self.cols(), wall_idx, cast_ranges)
    }

    pub fn cast_vertical_shadows(
        &self,
        wall_idx: usize,
        cast_ranges: (Option<usize>, Option<usize>),
    ) -> Vec<ShadowBlend> {
        ShadowBlend::pack_from(self.rows(), wall_idx, cast_ranges)
    }

    pub fn still_has_move(&self) -> bool {
        (0..self.columns - 1) // Vertical walls
            .map(|col| self.cast_vertical_shadows(col, (None, None)))
            .map(|shadows| Matcher::match_same(&shadows))
            .any(|couples| !couples.is_empty())
            || (1..self.rows - 1) // Horizontal walls
                .map(|row| self.cast_horizontal_shadows(row, (None, None)))
                .map(|shadows| Matcher::match_same(&shadows))
                .any(|couples| !couples.is_empty())
    }

    // Check 2 blocks if they are matching and return the connection
    pub fn connect(&self, blk1: &Block, blk2: &Block) -> Result<Vec<Block>, String> {
        if let Err(_err) = self.check_border_block(blk1) {
            panic!("Not a valid block to check, cannot check border");
        }
        if let Err(_err) = self.check_border_block(blk2) {
            panic!("Not a valid block to check, cannot check border");
        }
        let meme1 = self.cell(blk1);
        let meme2 = self.cell(blk2);
        if NO_MEME == meme1 || NO_MEME == meme2 {
            panic!("Cannot match with empty meme");
        }
        if self.cell(blk1) != self.cell(blk2) {
            panic!("Cannot connect different meme, wait for twist in future version then!");
        }
        let mut track = Track::new(*blk1, *blk2);
        while track.search(|blk| self.check_valid_empty_block(blk).is_ok()) {}
        if track.goal_found() {
            Ok(track.backtrace())
        } else {
            Err("Not found".to_string())
        }
    }
}
