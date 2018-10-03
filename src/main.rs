use ggez::{Context, GameResult};
use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod, MouseButton};
use ggez::graphics::{self, Color, DrawMode, Point2, Rect};

/// The target window size width/height wise.
const WINDOW_SIZE: f32 = 800.0;
/// The rank of the board, or how many squares make up a side.
const RANK: usize = 4;
/// The size of a single square on the board.
const SPACE_SIZE: f32 = WINDOW_SIZE / RANK as f32;
/// The radius of a player's piece.
const RADIUS: f32 = SPACE_SIZE / 3.0;

mod mainstate;
use self::mainstate::{MainState, Piece};

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.board.iter().any(Option::is_none) {
            self.check_winner();
        } else {
            //  If no valid move can be made, they forfeit their turn
            if !self.has_move() {
                self.next_turn();
            }

            //  If neither side has a valid move, then end the game
            if !self.has_move() {
                self.check_winner();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        
        let mut color_flag = false;
        //  Keep track of which index has the spot with the most captures
        let mut best_spot = None;

        for (index, piece) in self.board.iter().enumerate() {
            let col = (index % RANK) as f32;
            let row = (index / RANK) as f32;

            if col == 0.0 {
                color_flag = !color_flag;
            }

            if self.valid_space(col as usize, row as usize) {
                graphics::set_color(ctx, Color::from((255, 19, 22)))?;
                let total = self.captures(col as usize, row as usize).len();
                if best_spot.is_none() || total > best_spot.unwrap_or(0) {
                    best_spot = Some(index);
                }
            } else if color_flag {
                graphics::set_color(ctx, Color::from((158, 19, 22)))?;
            } else {
                graphics::set_color(ctx, Color::from((59, 122, 87)))?;
            }

            color_flag = !color_flag;

            graphics::rectangle(ctx,
                DrawMode::Fill,
                Rect::new(col * SPACE_SIZE, row * SPACE_SIZE, SPACE_SIZE, SPACE_SIZE))?;

            if let Some(p) = piece {
                if *p == Piece::Black {
                    graphics::set_color(ctx, graphics::BLACK)?;
                } else {
                    graphics::set_color(ctx, graphics::WHITE)?;
                }

                graphics::circle(ctx, 
                    DrawMode::Fill, 
                    Point2::new(col * SPACE_SIZE + SPACE_SIZE / 2.0, row * SPACE_SIZE + SPACE_SIZE / 2.0),
                    RADIUS,
                    0.0001)?;
            }
        }

        graphics::set_color(ctx, Color::from((0, 255, 0)))?;
        
        if let Some(best) = best_spot {
            graphics::rectangle(ctx,
                DrawMode::Fill,
                Rect::new((best % RANK) as f32 * SPACE_SIZE,
                    (best / RANK) as f32 * SPACE_SIZE,
                    SPACE_SIZE,
                    SPACE_SIZE))?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            Keycode::R => self.reset(),
            _ => {}
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        if button == MouseButton::Left {
            let pos_x = (x as f32 / SPACE_SIZE) as usize;
            let pos_y = (y as f32 / SPACE_SIZE) as usize;

            if self.valid_space(pos_x, pos_y) {
                self.place(pos_x, pos_y);
            }
        }
    }
}

fn main() {
    let size = (SPACE_SIZE as usize * RANK) as u32;
    let mut config = conf::Conf::new();
    config.window_mode.width = size;
    config.window_mode.height = size;

    let ctx = &mut Context::load_from_conf("Reversi", "Roughsketch", config).unwrap();

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("assets");
        ctx.filesystem.mount(&path, true);
    }

    let mut game = match MainState::new() {
        Ok(game) => game,
        Err(why) => {
            println!("Could not load MainState: {:?}", why);
            return;
        }
    };

    let result = event::run(ctx, &mut game);

    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
