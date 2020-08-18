extern crate core;
extern crate macroquad;
extern crate ui;

use core::prelude::*;
use macroquad::*;
use std::process::exit;
use ui::*;

// TODO: Use pokedex or internet meme as texture
fn meme_colors(meme: Meme) -> Color {
    match meme {
        1 => DARKGREEN,
        2 => RED,
        3 => ORANGE,
        4 => BLUE,
        5 => GRAY,
        6 => GREEN,
        7 => YELLOW,
        8 => PURPLE,
        9 => BROWN,
        10 => BEIGE,
        _ => BLANK,
    }
}

fn draw_meme(board: &GameBoard, pos: &Block, meme: Meme) {
    if meme == NO_MEME {
        return;
    }

    let region = board.calc_block_region(pos);

    let meme_x = region.coord.x + 2.;
    let meme_y = region.coord.y + 2.;
    let meme_width = region.size.width - 4.;
    let meme_height = region.size.height - 4.;

    let color = meme_colors(meme);
    draw_rectangle(meme_x, meme_y, meme_width, meme_height, color);
}

fn draw_game_board(board: &GameBoard, game_map: &GameMap) {
    for i in 0..game_map.width {
        for j in 0..game_map.height {
            let blk = Block { x: i, y: j };
            draw_meme(&board, &blk, game_map.cell(&blk));
        }
    }
}

fn draw_connector(board: &GameBoard, trace: &[Block]) {
    for triplet in trace.windows(2) {
        let first_region = board.calc_block_region(&triplet[0]);
        let first_center = Coordinate {
            x: first_region.coord.x + first_region.size.width / 2.,
            y: first_region.coord.y + first_region.size.height / 2.,
        };
        let second_region = board.calc_block_region(&triplet[1]);
        let second_center = Coordinate {
            x: second_region.coord.x + second_region.size.width / 2.,
            y: second_region.coord.y + second_region.size.height / 2.,
        };
        draw_circle(first_center.x, first_center.y, 2.5, RED);
        draw_circle(second_center.x, second_center.y, 2.5, RED);
        draw_line(
            first_center.x,
            first_center.y,
            second_center.x,
            second_center.y,
            5.,
            RED,
        );
    }
}

#[macroquad::main("meme-connect")]
async fn main() {
    // Seed random
    rand::srand((screen_width() + screen_height()) as u64);
    let mut game_map = GameMap::new(10, 10).unwrap();
    let playground = game_map.playground_blocks();
    let empty_blocks = game_map.collect_empty_blocks(&mut playground.iter());
    let mut interest_blocks = empty_blocks
        .iter()
        .filter(|blk| !(4..7).contains(&blk.x))
        .filter(|blk| !(4..7).contains(&blk.y))
        .copied()
        .collect::<Vec<&Block>>();
    let mut shuffled = Vec::new();
    while !interest_blocks.is_empty() {
        let idx = rand::gen_range(0, interest_blocks.len());
        let blk = interest_blocks.remove(idx);
        shuffled.push(blk);
    }
    game_map
        .set_meme_regions(&mut (1..11), &mut shuffled)
        .unwrap();
    game_map.set_meme(10, &Block { x: 4, y: 4 }).unwrap();
    game_map.set_meme(10, &Block { x: 6, y: 6 }).unwrap();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            exit(0);
        }
        clear_background(BLANK);
        let screen = RegionSize {
            width: screen_width(),
            height: screen_height(),
        };
        let board = GameBoard::new(&game_map, screen);
        draw_game_board(&board, &game_map);

        if is_mouse_button_down(MouseButton::Left) {
            let trace = game_map
                .connect(&Block { x: 4, y: 4 }, &Block { x: 6, y: 6 })
                .unwrap();
            draw_connector(&board, &trace);
        }
        next_frame().await;
    }
}
