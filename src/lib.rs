extern crate rand;

use rand::prelude::*;

pub type Meme = usize;
const NO_MEME: Meme = 0;

#[derive(Debug)]
pub struct ShadowTrace {
    meme: Meme,
    pos_on_track: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn rand(rng: &mut ThreadRng, from: &mut Vec<Point>) -> Point {
        from.remove(rng.gen_range(0, from.len()))
    }
}

pub struct GameMap {
    data: Vec<Vec<Meme>>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> GameMap {
        GameMap {
            data: vec![vec![NO_MEME; width]; height],
        }
    }

    pub fn _fmt(&self) -> String {
        self.data
            .iter()
            .map(|line| {
                format!(
                    "|{}|",
                    line.iter()
                        .map(|meme| format!("{:^5}", meme))
                        .collect::<Vec<String>>()
                        .join("|")
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn check_border_block(&self, pos: Point) -> Result<(), String> {
        if pos.x == 0 {
            return Err("border left".to_string());
        }
        if pos.x == self.data[0].len() - 1 {
            return Err("border right".to_string());
        }
        if pos.y == 0 {
            return Err("border top".to_string());
        }
        if pos.y == self.data.len() - 1 {
            return Err("border bottom".to_string());
        }
        Ok(())
    }

    pub fn collect_empty_blocks(&self) -> Vec<Point> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    // Only take empty block
                    .filter(|(_x, &val)| NO_MEME == val)
                    .map(move |(x, &_val)| Point { x, y })
            })
            .filter(|&block| {
                if let Ok(()) = self.check_border_block(block) {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<Point>>()
    }

    pub fn check_valid_empty_block(&self, pos: Point) -> Result<(), String> {
        if let Err(err) = self.check_border_block(pos) {
            return Err(format!("{:?} is at {}", pos, err));
        }
        match self.data[pos.y][pos.x] {
            NO_MEME => Ok(()),
            meme => Err(format!("{:?} occupied with {}", pos, meme)),
        }
    }

    pub fn set_couple(&mut self, meme: Meme, pos1: Point, pos2: Point) -> Result<(), String> {
        for &pos in [pos1, pos2].iter() {
            if let Err(err) = self.check_valid_empty_block(pos) {
                return Err(err);
            }
        }
        self.data[pos1.y][pos1.x] = meme;
        self.data[pos2.y][pos2.x] = meme;
        Ok(())
    }
}

pub fn shadow_tracing<'a, TrackIterable>(
    on_track: &mut TrackIterable,
    shadow_position: usize,
    look_up_to_postion: usize,
) -> Option<ShadowTrace>
where
    TrackIterable: IntoIterator<Item = &'a Meme>
        + DoubleEndedIterator<Item = &'a Meme>
        + ExactSizeIterator<Item = &'a Meme>,
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
        match shadow_tracing(&mut TRACK.iter(), wall, look_up_to) {
            Some(shadow) => println!("{:#?}", shadow),
            None => println!("Cannot find subject in range that can cast shadow!"),
        }
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
        let shadow = shadow_tracing(&mut TRACK.iter(), wall, look_up_to).unwrap();
        println!("{:#?}", shadow);
        assert_eq!(shadow.pos_on_track, wall);
        assert_eq!(shadow.meme, TRACK[wall]);
    }

    #[test]
    fn test_wall_meets_subject() {
        println!("Track: {:?}", TRACK);
        println!("Wall position: 1, look up position: 1");
        shadow_tracing(&mut TRACK.iter(), 1, 1).unwrap();
    }
}
