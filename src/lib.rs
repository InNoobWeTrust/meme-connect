extern crate rand;

use rand::prelude::*;

pub type Meme = usize;
const NO_MEME: Meme = 0;

pub type GameMap = Vec<Vec<Meme>>;

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
    pub fn random(rng: &mut ThreadRng, from: Meme, to_exclusive: Meme) -> Self {
        Point {
            x: rng.gen_range(from, to_exclusive),
            y: rng.gen_range(from, to_exclusive),
        }
    }
}

pub fn empty_map(width: usize, height: usize) -> GameMap {
    vec![vec![NO_MEME; width]; height]
}

pub fn _game_map_fmt(game_map: &GameMap) -> String {
    game_map
        .iter()
        .map(|line| format!("{:?}", line))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn map_check_border_pos(game_map: &GameMap, pos: Point) -> Result<(), String> {
    if pos.x == 0 {
        return Err("border left".to_string());
    }
    if pos.x == game_map[0].len() - 1 {
        return Err("border right".to_string());
    }
    if pos.y == 0 {
        return Err("border top".to_string());
    }
    if pos.y == game_map.len() - 1 {
        return Err("border bottom".to_string());
    }
    Ok(())
}

pub fn map_check_dual_pos(game_map: &GameMap, pos1: Point, pos2: Point) -> Result<(), String> {
    if let Err(err) = map_check_border_pos(&game_map, pos1) {
        return Err(format!("pos1 {:?} is at {}", pos1, err));
    }
    if let Err(err) = map_check_border_pos(&game_map, pos2) {
        return Err(format!("pos2 {:?} is at {}", pos2, err));
    }
    let old_pos1 = game_map[pos1.y][pos1.x];
    let old_pos2 = game_map[pos2.y][pos2.x];
    match (old_pos1, old_pos2) {
        (NO_MEME, NO_MEME) => Ok(()),
        (meme1, NO_MEME) => Err(format!("Position 1 occupied with {}", meme1)),
        (NO_MEME, meme2) => Err(format!("Position 2 occupied with {}", meme2)),
        (meme1, meme2) => Err(format!(
            "Both position occupied with {} and {}",
            meme1, meme2
        )),
    }
}

pub fn map_set_couple(
    game_map: &mut GameMap,
    meme: Meme,
    pos1: Point,
    pos2: Point,
) -> Result<(), String> {
    if let Err(err) = map_check_dual_pos(&game_map, pos1, pos2) {
        return Err(err);
    }
    game_map[pos1.y][pos1.x] = meme;
    game_map[pos2.y][pos2.x] = meme;
    Ok(())
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
