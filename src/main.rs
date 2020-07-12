extern crate rand;

extern crate meme_connect;

use rand::prelude::*;

use meme_connect::*;

fn main() {
    let mut rng = thread_rng();
    let mut game_map = GameMap::new(10, 10).unwrap();
    println!("Game map:\n{}", game_map._fmt());
    let playground = game_map.playground_blocks();
    let empty_blocks = game_map.collect_empty_blocks(&mut playground.iter());
    let mut interest_blocks = empty_blocks
        .iter()
        .filter(|blk| !(4..6).contains(&blk.x))
        .filter(|blk| !(4..6).contains(&blk.y))
        .map(|&blk| blk)
        .collect::<Vec<&Block>>();
    if let Err(err) =
        game_map.set_meme_regions(&mut (1usize..).into_iter(), &mut interest_blocks, &mut rng)
    {
        panic!("Failed to set meme in regions: {}", err);
    }
    println!("Game map after filling some couples:\n{}", game_map._fmt());
    let vertical_shadows = game_map.cast_vertical_shadows(4, game_map.width);
    println!("Vertical shadows:\n{:#?}", vertical_shadows);
    let horizontal_shadows = game_map.cast_horizontal_shadows(4, game_map.height);
    println!("Horizontal shadows:\n{:#?}", horizontal_shadows);
}
