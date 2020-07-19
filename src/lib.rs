extern crate rand;

use rand::prelude::*;
use std::{
    iter::{Skip, StepBy, Take},
    slice::Iter,
};

pub type Meme = usize;
pub const NO_MEME: Meme = 0;

#[derive(Debug)]
pub struct ShadowTrace {
    pub meme: Meme,
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

    fn col<'a>(&'a self, index: usize) -> StepBy<Skip<Iter<'a, usize>>> {
        self.data.iter().skip(index).step_by(self.width)
    }

    fn row<'a>(&'a self, index: usize) -> Take<Skip<Iter<'a, usize>>> {
        self.data.iter().skip(index * self.height).take(self.height)
    }

    fn rows<'a>(&'a self) -> Vec<Take<Skip<Iter<'a, usize>>>> {
        (0..self.height).map(|y| self.row(y)).collect::<Vec<_>>()
    }

    fn cols<'a>(&'a self) -> Vec<StepBy<Skip<Iter<'a, usize>>>> {
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

    /// After casting shadows on a wall, if 2 shadows with the same type have
    /// no blocking shadow(s) in between, then there is a match
    pub fn connect_shadows<'a, T>(shadow_wall: &'a mut T) -> Vec<(usize, usize)>
    where
        T: Iterator<Item = &'a Option<ShadowTrace>>,
    {
        let ungap = shadow_wall
            .enumerate()
            .filter_map(|(idx, possible_shadow)| {
                if let Some(shadow) = possible_shadow {
                    Some((idx, shadow))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, &ShadowTrace)>>();
        ungap
            .windows(2)
            .filter_map(|w: &[(usize, &ShadowTrace)]| {
                if w[0].1.meme == w[1].1.meme {
                    Some((w[0].0, w[1].0))
                } else {
                    None
                }
            })
            .collect()
    }

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
                //.inspect(|&(pos, &meme)| println!("=> tracing step: pos={}, meme={:?}", pos, meme))
                .find(|&(_pos, &meme)| meme != NO_MEME)
        } else {
            on_track
                .enumerate()
                .skip(look_up_to_postion)
                .rev()
                .skip_while(|&(pos, &_meme)| pos > shadow_position)
                //.inspect(|&(pos, &meme)| println!("=> tracing step: pos={}, meme={:?}", pos, meme))
                .find(|&(_pos, &meme)| meme != NO_MEME)
        } {
            return Some(ShadowTrace { meme, pos_on_track });
        }
        None
    }

    pub fn cast_horizontal_shadows(
        &self,
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

    pub fn cast_vertical_shadows(
        &self,
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

    pub fn still_has_move(&self) -> bool {
        (2..self.width - 2) // Vertical wall low vision (range 1, whatever on wall)
            .map(|col| self.cast_vertical_shadows(col, col))
            .map(|shadows| GameMap::connect_shadows(&mut shadows.iter()))
            .any(|couples| !couples.is_empty())
            || (0..self.width - 3) // Look to the right
                .map(|col| self.cast_vertical_shadows(col, self.width - 2))
                .map(|shadows| GameMap::connect_shadows(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
            || (2..self.width - 1) // Look to the left
                .map(|col| self.cast_vertical_shadows(col, 1))
                .map(|shadows| GameMap::connect_shadows(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
            || (2..self.height - 2) // Horizontal wall low vision (range 1, whatever on wall)
                .map(|row| self.cast_horizontal_shadows(row, row))
                .map(|shadows| GameMap::connect_shadows(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
            || (0..self.height - 3) // Look down
                .map(|row| self.cast_horizontal_shadows(row, self.height - 2))
                .map(|shadows| GameMap::connect_shadows(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
            || (2..self.width - 1) // Look up
                .map(|row| self.cast_horizontal_shadows(row, 1))
                .map(|shadows| GameMap::connect_shadows(&mut shadows.iter()))
                .any(|couples| !couples.is_empty())
    }
}

#[cfg(test)]
mod test_tracing_shadow {
    use crate::*;

    #[test]
    fn test_shadow_connections() -> Result<(), String> {
        let mut game_map = GameMap::new(6, 3).unwrap();
        println!("Game map:\n{}", game_map._fmt());
        game_map.set_meme(1, &Block::new(1, 1))?;
        game_map.set_meme(1, &Block::new(4, 1))?;
        println!("Game map after filling some couples:\n{}", game_map._fmt());
        let shadows = game_map.cast_horizontal_shadows(0, game_map.width);
        let couples = GameMap::connect_shadows(&mut shadows.iter());
        println!("couples: {:?}", couples);
        assert_eq!(couples, &[(1, 4)]);
        Ok(())
    }

    #[test]
    fn test_wall_subject() {
        const TRACK: &[Meme] = &[1, 0, 0, 0, 0];
        println!("Track: {:?}", TRACK);
        let wall = TRACK.len();
        let look_up_to = 0;
        println!("Wall position: {}, look up position: {}", wall, look_up_to);
        let trace = GameMap::shadow_tracing(&mut TRACK.iter(), wall, look_up_to).unwrap();
        assert_eq!(trace.pos_on_track, 0);
        assert_eq!(trace.meme, 1);
    }

    #[test]
    fn test_limited_vision() {
        const TRACK: &[Meme] = &[1, 0, 0, 0, 0];
        println!("Track: {:?}", TRACK);
        let wall = TRACK.len();
        let look_up_to = 1;
        println!("Wall position: {}, look up position: {}", wall, look_up_to);
        let possible_trace = GameMap::shadow_tracing(&mut TRACK.iter(), wall, look_up_to);
        assert_eq!(possible_trace.is_none(), true);
    }

    #[test]
    fn test_wall_meets_subject() {
        const TRACK: &[Meme] = &[1, 0, 2, 0, 0];
        println!("Track: {:?}", TRACK);
        println!("Wall position: 1, look up position: 1");
        let trace = GameMap::shadow_tracing(&mut TRACK.iter(), 2, 2).unwrap();
        assert_eq!(trace.pos_on_track, 2);
        assert_eq!(trace.meme, 2);
    }
}

pub fn main() {}
