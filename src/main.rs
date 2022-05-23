use ::board::prelude::*;
use ::macroquad::prelude::*;
use ::ui::prelude::*;
use std::process::exit;

const TRANSPARENT: Color = Color {
    r: 255.,
    g: 255.,
    b: 255.,
    a: 255.,
};

struct Drawer;

impl Drawer {
    fn draw_loading_screen() {
        let loading_string = "Loading...";
        let loading_string_size = measure_text(loading_string, None, 36, 1.);
        draw_text(
            "Loading",
            screen_width() / 2. - loading_string_size.width,
            screen_height() / 2. - loading_string_size.height,
            36.,
            WHITE,
        );
    }

    fn draw_connector(board: &Region, trace: &[Cell]) {
        for pair in trace.windows(2) {
            let first_center = board.cell_region(&pair[0]).unwrap().center();
            let second_center = board.cell_region(&pair[1]).unwrap().center();
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

    fn draw_conquered(board: &Region, paths: &[Path]) {
        for path in paths {
            path.steps().for_each(|cell| {
                let center = board.cell_region(&cell).unwrap().center();
                draw_circle(center.x, center.y, 10., GREEN);
            })
        }
    }
}

struct BoardDrawer {
    sprite_sheet: Texture2D,
}

impl BoardDrawer {
    async fn new() -> Self {
        BoardDrawer {
            // Load textures
            sprite_sheet: load_texture("assets/poke_28x25+21_32x32.png")
                .await
                .unwrap(),
        }
    }

    fn sprite2rect(sprite_id: SpriteId) -> Option<Rect> {
        const MAX_SPRITES: usize = 721;
        const COLUMNS: usize = 28;
        const SPRITE_SIZE: f32 = 32.;

        if sprite_id >= MAX_SPRITES {
            return None;
        }

        let col = sprite_id % COLUMNS;
        let row = sprite_id / COLUMNS;

        Some(Rect::new(
            col as f32 * SPRITE_SIZE,
            row as f32 * SPRITE_SIZE,
            SPRITE_SIZE,
            SPRITE_SIZE,
        ))
    }

    fn draw_sprite(&self, board: &Region, cell: &Cell, sprite: SpriteId) {
        if sprite == NO_SPRITE {
            return;
        }

        let region = board.cell_region(&cell).unwrap();

        let sprite_x = region.coord.x + 2.;
        let sprite_y = region.coord.y + 2.;
        let sprite_width = region.size.width - 4.;
        let sprite_height = region.size.height - 4.;

        // Border
        draw_rectangle(sprite_x, sprite_y, sprite_width, sprite_height, LIGHTGRAY);

        // Background filler
        draw_rectangle(
            sprite_x + 4.,
            sprite_y + 4.,
            sprite_width - 8.,
            sprite_height - 8.,
            DARKGRAY,
        );

        // Texture
        draw_texture_ex(
            self.sprite_sheet,
            sprite_x,
            sprite_y,
            TRANSPARENT,
            DrawTextureParams {
                dest_size: Some(vec2(sprite_width, sprite_height)),
                source: Self::sprite2rect(sprite),
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    fn draw_select_border(&self, board: &Region, cell: &Cell) {
        let region = board.cell_region(&cell).unwrap();

        let x = region.coord.x;
        let y = region.coord.y;
        let w = region.size.width;
        let h = region.size.height;

        draw_rectangle_lines(x, y, w, h, 8., RED);
    }

    fn draw_board(&self, board: &Region, mapping: &Mapping) {
        for i in 0..mapping.columns {
            for j in 0..mapping.rows {
                let cell = Cell { column: i, row: j };
                self.draw_sprite(board, &cell, mapping.get_sprite(&cell));
            }
        }
    }
}

#[derive(Default)]
struct Interaction {
    mouse_hold: Option<f64>,
}

impl Interaction {
    fn get_click(&mut self) -> Option<(f32, f32)> {
        match (self.mouse_hold, is_mouse_button_down(MouseButton::Left)) {
            (None, true) => {
                self.mouse_hold = Some(get_time());
                None
            }
            (Some(instant), false) => {
                let elapsed = get_time() - instant;
                // Debounce
                let should_trigger = elapsed > 0.01f64 && elapsed < 1.0f64;
                self.mouse_hold = None;
                if should_trigger {
                    Some(mouse_position())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn check_exit(&self) {
        if is_key_pressed(KeyCode::Escape) {
            exit(0);
        }
    }
}

#[macroquad::main("meme-connect")]
async fn main() {
    Drawer::draw_loading_screen();
    // Seed random
    rand::srand((get_time() * 1000.) as u64);
    let mut mapping = Mapping::new(16, 16);
    let playground = mapping.mutable_cells();
    let mut fillable_cells: Vec<Cell> = playground
        .filter(|cell| mapping.check_fillable_cell(cell).is_ok())
        .collect();
    let mut shuffled = Vec::new();
    for _ in 0..64 {
        if fillable_cells.is_empty() {
            break;
        }
        let idx = rand::gen_range(0, fillable_cells.len());
        let cell = fillable_cells.remove(idx);
        shuffled.push(cell);
    }
    mapping.fill_regions(&mut shuffled, 1..).unwrap();

    let board_drawer = BoardDrawer::new().await;
    let mut interaction: Interaction = Default::default();
    let mut connector: CellConnector = CellConnector::new();
    let mut debug_paths: Vec<Path> = Vec::new();
    loop {
        //-------------------------------------------------Check exit condition
        interaction.check_exit();
        if mapping.no_more_move() {
            exit(0);
        }

        //--------------------------Update current screen size and board region
        let screen = Size {
            width: screen_width(),
            height: screen_height(),
        };
        let board = screen.game_board_region(mapping.columns, mapping.rows);

        //-------------------------------------------Update interaction (click)
        if let Some((x, y)) = interaction.get_click() {
            let click_coord = Coordinate { x, y };
            if let Ok(cell) = board.cell_from_coord(&click_coord) {
                if NO_SPRITE != mapping.get_sprite(&cell) {
                    connector.select(cell);
                }
            }
        }

        //-----------------------------------------Update connector and mapping
        if let Err(mut conquered) = connector.update(&mut mapping, get_time()) {
            debug_paths.clear();
            debug_paths.append(&mut conquered);
        }

        //---------------------------------------------------------------Render
        // clear screen
        clear_background(BLANK);
        // Draw current board after update
        board_drawer.draw_board(&board, &mapping);

        // Draw border for current selection
        if let Some(&cell) = connector.get_selection() {
            board_drawer.draw_select_border(&board, &cell)
        }

        // Draw pending destroy couples
        for couple in connector
            .poll_destroying()
            .iter()
            .filter(|couple| get_time() - couple.epoch < 0.5)
        {
            for (cell, sprite_id) in couple.remnants.iter() {
                board_drawer.draw_sprite(&board, &cell, *sprite_id);
            }
            Drawer::draw_connector(&board, &couple.nodes);
        }

        // Draw debug points
        Drawer::draw_conquered(&board, &debug_paths);

        //--------------------------------------------------Wait for next frame
        next_frame().await;
    }
}
