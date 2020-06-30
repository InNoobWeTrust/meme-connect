extern crate rand;

use rand::prelude::*;

use meme_connect::*;

fn main() {
    let mut rng = thread_rng();
    let mut game_map = empty_map(10, 10);
    println!("Game map:\n{}", _game_map_fmt(&game_map));
    for meme in 1..=5 {
        while let Err(err) = map_set_couple(
            &mut game_map,
            meme,
            Point::random(&mut rng, 1, 10),
            Point::random(&mut rng, 1, 10),
        ) {
            println!("Trying again due to error: {}", err);
        }
    }
    println!(
        "Game map after filling some couples:\n{}",
        _game_map_fmt(&game_map)
    );
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
