extern crate rand;

use rand::prelude::*;
use std::{
    iter::{Skip, StepBy, Take},
    slice::Iter,
};

pub type Meme = usize;
const NO_MEME: Meme = 0;

#[derive(Debug)]
pub struct ShadowTrace {
    meme: Meme,
    pos_on_track: usize,
}

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

pub struct GameMap {
    pub width: usize,
    pub height: usize,
    data: Vec<Meme>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Option<GameMap> {
        if 0 != width && 0 != height && 0 == (width * height) % 2 {
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

    fn col<'a>(&'a self, index: usize) -> StepBy<Skip<Iter<'a, usize>>> {
        self.data.iter().skip(index).step_by(self.width)
    }

    fn row<'a>(&'a self, index: usize) -> Take<Skip<Iter<'a, usize>>> {
        self.data.iter().skip(index * self.height).take(self.height)
    }

    fn rows<'a>(&'a self) -> Vec<Take<Skip<Iter<'a, usize>>>> {
        (0..self.height)
            .into_iter()
            .map(|y| self.row(y))
            .collect::<Vec<_>>()
    }

    fn cols<'a>(&'a self) -> Vec<StepBy<Skip<Iter<'a, usize>>>> {
        (0..self.width)
            .into_iter()
            .map(|x| self.col(x))
            .collect::<Vec<_>>()
    }

    pub fn playground_blocks(&self) -> Vec<Block> {
        (1..self.width - 1)
            .into_iter()
            .flat_map(move |x| {
                (1..self.height - 1)
                    .into_iter()
                    .map(move |y| Block::new(x, y))
            })
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

    pub fn collect_empty_blocks<'a>(
        &self,
        from: &mut dyn Iterator<Item = &'a Block>,
    ) -> Vec<&'a Block> {
        from.filter(|blk| self.check_valid_empty_block(blk).is_ok())
            .collect::<Vec<_>>()
    }

    pub fn set_meme(&mut self, meme: Meme, blk: &Block) -> Result<(), String> {
        self.check_valid_empty_block(blk)?;
        let idx = self.block2idx(blk);
        self.data[idx] = meme;
        Ok(())
    }

    pub fn set_meme_regions(
        &mut self,
        meme_lst: &mut dyn Iterator<Item = Meme>,
        regions: &mut Vec<&Block>,
        rng: &mut ThreadRng,
    ) -> Result<(), String> {
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

    pub fn shadow_tracing<'a, T>(
        on_track: &mut T,
        shadow_position: usize,
        look_up_to_postion: usize,
    ) -> Option<ShadowTrace>
    where
        T: DoubleEndedIterator<Item = &'a Meme> + ExactSizeIterator<Item = &'a Meme>,
    {
        if let Some((pos_on_track, &meme)) = if shadow_position <= look_up_to_postion {
            on_track
                .enumerate()
                .skip(shadow_position)
                .take(look_up_to_postion - shadow_position + 1)
                .inspect(|&(pos, &meme)| println!("=> tracing step: pos={}, meme={:?}", pos, meme))
                .find(|&(_pos, &meme)| meme != NO_MEME)
        } else {
            on_track
                .enumerate()
                .skip(look_up_to_postion)
                .rev()
                .skip_while(|&(pos, &_meme)| pos > shadow_position)
                .inspect(|&(pos, &meme)| println!("=> tracing step: pos={}, meme={:?}", pos, meme))
                .find(|&(_pos, &meme)| meme != NO_MEME)
        } {
            return Some(ShadowTrace { meme, pos_on_track });
        }
        None
    }

    pub fn cast_horizontal_shadows<'a>(
        &'a self,
        horizontal_wall_idx: usize,
        look_up_to_horizontal_idx: usize,
    ) -> Vec<Option<ShadowTrace>> {
        self.cols()
            .into_iter()
            .map(|mut col| {
                GameMap::shadow_tracing(&mut col, horizontal_wall_idx, look_up_to_horizontal_idx)
            })
            .collect::<_>()
    }

    pub fn cast_vertical_shadows<'a>(
        &'a self,
        vertical_wall_idx: usize,
        look_up_to_vertical_idx: usize,
    ) -> Vec<Option<ShadowTrace>> {
        self.rows()
            .into_iter()
            .map(|mut row| {
                GameMap::shadow_tracing(&mut row, vertical_wall_idx, look_up_to_vertical_idx)
            })
            .collect::<_>()
    }
}

#[cfg(test)]
mod test_tracing_shadow {
    extern crate rand;

    use crate::*;

    const TRACK: &[Meme] = &[1, 2, 0, 3, 0, 0, 4, 0, 0, 5];

    #[test]
    fn test_wall_subject_different_position() {
        println!("Track: {:?}", TRACK);
        let mut rng = thread_rng();
        let mut wall = rng.gen_range(0, TRACK.len() - 1);
        while TRACK[wall] != 0 {
            wall = rng.gen_range(0, TRACK.len() - 1);
        }
        let mut look_up_to = rng.gen_range(0, TRACK.len() - 1);
        while look_up_to == wall {
            look_up_to = rng.gen_range(0, TRACK.len() - 1);
        }
        println!("Wall position: {}, look up position: {}", wall, look_up_to);
        GameMap::shadow_tracing(&mut TRACK.iter(), wall, look_up_to).unwrap();
    }

    #[test]
    fn test_wall_blocked() {
        println!("Track: {:?}", TRACK);
        let mut rng = thread_rng();
        let mut wall = rng.gen_range(0, TRACK.len() - 1);
        while TRACK[wall] == 0 {
            wall = rng.gen_range(0, TRACK.len() - 1);
        }
        let look_up_to = rng.gen_range(0, TRACK.len() - 1);
        println!("Wall position: {}, look up position: {}", wall, look_up_to);
        let shadow = GameMap::shadow_tracing(&mut TRACK.iter(), wall, look_up_to).unwrap();
        println!("{:#?}", shadow);
        assert_eq!(shadow.pos_on_track, wall);
        assert_eq!(shadow.meme, TRACK[wall]);
    }

    #[test]
    fn test_wall_meets_subject() {
        println!("Track: {:?}", TRACK);
        println!("Wall position: 1, look up position: 1");
        GameMap::shadow_tracing(&mut TRACK.iter(), 1, 1).unwrap();
    }
}
