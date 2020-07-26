extern crate rand;

extern crate meme_connect;

use rand::prelude::*;

use meme_connect::prelude::*;

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
    game_map.set_meme(10, &Block::new(6, 6))?;
    println!("Game map after filling some couples:\n{}", game_map._fmt());
    let vertical_shadows = game_map.cast_vertical_shadows(5, (None, None));
    println!(
        "Vertical shadows:\n{:?}",
        vertical_shadows
            .iter()
            .map(|blend| blend.memes())
            .collect::<Vec<_>>()
    );
    let vertical_couples = Matcher::match_same(&vertical_shadows);
    println!("Vertical couples: {:?}", &vertical_couples);
    let horizontal_shadows = game_map.cast_horizontal_shadows(5, (None, None));
    println!(
        "Horizontal shadows:\n{:?}",
        horizontal_shadows
            .iter()
            .map(|blend| blend.memes())
            .collect::<Vec<_>>()
    );
    let horizontal_couples = Matcher::match_same(&horizontal_shadows);
    println!("Horizontal couples: {:?}", &horizontal_couples);
    let check = game_map.still_has_move();
    println!("Is there any more moves? => {}", check);
    Ok(())
}
