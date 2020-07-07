extern crate rand;

extern crate meme_connect;

use rand::prelude::*;

use meme_connect::*;

fn main() {
    let mut rng = thread_rng();
    let mut game_map = GameMap::new(12, 12);
    println!("Game map:\n{}", game_map._fmt());
    let mut empty_points = game_map.collect_empty_blocks();
    for meme in 1.. {
        if empty_points.is_empty() {
            break;
        }
        while let Err(err) = game_map.set_couple(
            meme,
            Point::rand(&mut rng, &mut empty_points),
            Point::rand(&mut rng, &mut empty_points),
        ) {
            println!("Trying again due to error: {}", err);
        }
    }
    println!("Game map after filling some couples:\n{}", game_map._fmt());
    const TRACK: &[Meme] = &[1, 2, 0, 3, 0, 0, 4, 0, 0, 5];
    println!("Track: {:?}", TRACK);
    let wall = rng.gen_range(0, TRACK.len() - 1);
    let look_up_to = rng.gen_range(0, TRACK.len() - 1);
    println!("Wall position: {}, look up position: {}", wall, look_up_to);
    match shadow_tracing(&mut TRACK.iter(), wall, look_up_to) {
        Some(shadow) => println!("{:#?}", shadow),
        None => println!("Cannot find subject in range that can cast shadow!"),
    }
}
