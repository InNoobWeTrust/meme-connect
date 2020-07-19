extern crate rand;

extern crate meme_connect;

use rand::prelude::*;

use meme_connect::*;

fn main() -> Result<(), String> {
    let mut rng = thread_rng();
    let mut game_map = GameMap::new(10, 10).unwrap();
    println!("Game map:\n{}", game_map._fmt());
    let playground = game_map.playground_blocks();
    let empty_blocks = game_map.collect_empty_blocks(&mut playground.iter());
    let mut interest_blocks = empty_blocks
        .iter()
        .filter(|blk| !(4..7).contains(&blk.x))
        .filter(|blk| !(4..7).contains(&blk.y))
        .copied()
        .collect::<Vec<&Block>>();
    if let Err(err) = game_map.set_meme_regions(&mut (1usize..), &mut interest_blocks, &mut rng) {
        panic!("Failed to set meme in regions: {}", err);
    }
    game_map.set_meme(10, &Block::new(4, 4))?;
    game_map.set_meme(10, &Block::new(4, 6))?;
    game_map.set_meme(10, &Block::new(6, 4))?;
    game_map.set_meme(10, &Block::new(6, 6))?;
    println!("Game map after filling some couples:\n{}", game_map._fmt());
    let vertical_shadows = game_map.cast_vertical_shadows(5, game_map.width);
    println!(
        "Vertical shadows from the right:\n{:?}",
        vertical_shadows
            .iter()
            .map(|shadow| match &shadow.as_ref() {
                Some(trace) => trace.meme,
                None => NO_MEME,
            })
            .collect::<Vec<Meme>>()
    );
    let horizontal_shadows = game_map.cast_horizontal_shadows(5, game_map.height);
    println!(
        "Horizontal shadows from bottom:\n{:?}",
        horizontal_shadows
            .iter()
            .map(|shadow| match &shadow.as_ref() {
                Some(trace) => trace.meme,
                None => NO_MEME,
            })
            .collect::<Vec<Meme>>()
    );
    let couples = GameMap::connect_shadows(&mut vertical_shadows.iter());
    println!("couples: {:?}", couples);
    let check = game_map.still_has_move();
    println!("Is there any more moves? => {}", check);
    Ok(())
}
