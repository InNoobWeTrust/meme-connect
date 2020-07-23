extern crate rand;

use rand::prelude::*;

use crate::{block::Block, matcher::Matcher, meme::*, shadow::ShadowTrace};

use std::{
    iter::{Skip, StepBy, Take},
    slice::Iter,
};

pub struct GameMap {
    pub width: usize,
    pub height: usize,
    data: Vec<Meme>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Option<GameMap> {
        if 0 != width && 0 != height && 0 == ((width - 2) * (height - 2)) % 2 {
            Some(GameMap {
                width,
                height,
                data: vec![NO_MEME; width * height],
            })
        } else {
            None
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    ////////////////////////////// View of data ///////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    pub fn cell<'a>(&'a self, blk: &Block) -> Option<&'a Meme> {
        self.row(blk.y).nth(blk.x)
    }

    fn col<'a>(&'a self, index: usize) -> StepBy<Skip<Iter<'a, Meme>>> {
        self.data.iter().skip(index).step_by(self.width)
    }

    fn row<'a>(&'a self, index: usize) -> Take<Skip<Iter<'a, Meme>>> {
        self.data.iter().skip(index * self.width).take(self.width)
    }

    fn rows<'a>(&'a self) -> Vec<Take<Skip<Iter<'a, Meme>>>> {
        (0..self.height).map(|y| self.row(y)).collect::<Vec<_>>()
    }

    fn cols<'a>(&'a self) -> Vec<StepBy<Skip<Iter<'a, Meme>>>> {
        (0..self.width).map(|x| self.col(x)).collect::<Vec<_>>()
    }

    pub fn playground_blocks(&self) -> Vec<Block> {
        (1..self.width - 1)
            .flat_map(move |x| (1..self.height - 1).map(move |y| Block::new(x, y)))
            .collect::<Vec<_>>()
    }

    pub fn _fmt(&self) -> String {
        self.rows()
            .iter_mut()
            .map(|line| {
                format!(
                    "|{}|",
                    line.map(|meme| format!("{:^5}", meme))
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
        idx < self.width ||
            // Bottom border
            idx > self.width * self.height - 1 ||
            // Left border
            idx % self.width == 0 ||
            // Right border
            idx % self.width == self.width - 1
    }

    fn block2idx(&self, blk: &Block) -> usize {
        blk.y * self.width + blk.x
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

    pub fn set_meme_regions<T>(
        &mut self,
        meme_lst: &mut T,
        regions: &mut Vec<&Block>,
        rng: &mut ThreadRng,
    ) -> Result<(), String>
    where
        T: Iterator<Item = Meme>,
    {
        regions.shuffle(rng);
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
        horizontal_wall_idx: usize,
        look_up_to_horizontal_idx: usize,
    ) -> Vec<Option<ShadowTrace>> {
        ShadowTrace::trace_multi(
            &mut self.cols().into_iter(),
            horizontal_wall_idx,
            look_up_to_horizontal_idx,
        )
    }

    pub fn cast_vertical_shadows(
        &self,
        vertical_wall_idx: usize,
        look_up_to_vertical_idx: usize,
    ) -> Vec<Option<ShadowTrace>> {
        ShadowTrace::trace_multi(
            &mut self.rows().into_iter(),
            vertical_wall_idx,
            look_up_to_vertical_idx,
        )
    }

    pub fn still_has_move(&self) -> bool {
        (2..self.width - 2) // Vertical wall low vision (range 1, whatever on wall)
            .map(|col| self.cast_vertical_shadows(col, col))
            .map(|shadows| Matcher::match_same(&mut shadows.iter()))
            .any(|couples| !couples.is_empty())
            || (0..self.width - 3) // Look to the right
                .map(|col| self.cast_vertical_shadows(col, self.width - 2))
                .map(|shadows| Matcher::match_same(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
            || (2..self.width - 1) // Look to the left
                .map(|col| self.cast_vertical_shadows(col, 1))
                .map(|shadows| Matcher::match_same(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
            || (2..self.height - 2) // Horizontal wall low vision (range 1, whatever on wall)
                .map(|row| self.cast_horizontal_shadows(row, row))
                .map(|shadows| Matcher::match_same(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
            || (0..self.height - 3) // Look down
                .map(|row| self.cast_horizontal_shadows(row, self.height - 2))
                .map(|shadows| Matcher::match_same(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
            || (2..self.width - 1) // Look up
                .map(|row| self.cast_horizontal_shadows(row, 1))
                .map(|shadows| Matcher::match_same(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
    }
}
