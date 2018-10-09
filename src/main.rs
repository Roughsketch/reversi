use ggez::{Context, GameResult};
use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, MeshBuilder, Point2, Text};
use ggez::timer;
use parking_lot::RwLock;
use rayon::prelude::*;

use std::sync::Arc;

/// The target window size width/height wise.
const WINDOW_SIZE: f32 = 400.0;
/// The rank of the board, or how many squares make up a side.
const RANK: usize = 50;
/// The size of a single square on the board.
const SPACE_SIZE: f32 = WINDOW_SIZE / RANK as f32;
/// The radius of a player's piece.
const RADIUS: f32 = SPACE_SIZE / 3.0;

mod mainstate;
use self::mainstate::{MainState, Piece, Winner};

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.turns == RANK * RANK {
            match self.check_winner() {
                Winner::White => println!("White wins"),
                Winner::Black => println!("Black wins"),
                Winner::Tie => println!("Tie"),
            }

            if self.is_auto() {
                self.reset();
            }
        } else if self.is_auto() {
            let mut rng = rand::thread_rng();

            let candidates = (0..RANK*RANK).into_par_iter().filter(|index| {
                self.valid_space(index % RANK, index / RANK)
            }).collect::<Vec<usize>>();

            if let Ok(choice) = rand::seq::sample_iter(&mut rng, candidates, 1) {
                if choice.len() > 0 {
                    let index = choice[0];
                    self.place(index % RANK, index / RANK);
                } else {
                    self.next_turn();

                    //  If neither side has a valid move, then end the game
                    if !self.has_move() {
                        match self.check_winner() {
                            Winner::White => println!("White wins"),
                            Winner::Black => println!("Black wins"),
                            Winner::Tie => println!("Tie"),
                        }
                        self.reset();
                    }
                }
            } else {
                match self.check_winner() {
                    Winner::White => println!("White wins"),
                    Winner::Black => println!("Black wins"),
                    Winner::Tie => println!("Tie"),
                }
                self.reset();
            }
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

        graphics::draw_ex(ctx, &self.grid,
            DrawParam {
                color: Some(Color::from((158, 19, 22))),
                .. Default::default()
            })?;

        let black = Arc::new(RwLock::new(MeshBuilder::new()));
        let white = Arc::new(RwLock::new(MeshBuilder::new()));

        self.board
            .par_iter()
            .enumerate()
            .filter_map(|(i, &x)| {
                if let Some(piece) = x {
                    Some((i, piece))
                } else {
                    None
                }
            }).for_each(|(index, piece)| {
                let col = (index % RANK) as f32;
                let row = (index / RANK) as f32;

                if piece == Piece::Black {
                    black.write().circle(DrawMode::Fill, 
                        Point2::new(col * SPACE_SIZE + SPACE_SIZE / 2.0, row * SPACE_SIZE + SPACE_SIZE / 2.0),
                        RADIUS,
                        100.0);
                } else {
                    white.write().circle(DrawMode::Fill, 
                        Point2::new(col * SPACE_SIZE + SPACE_SIZE / 2.0, row * SPACE_SIZE + SPACE_SIZE / 2.0),
                        RADIUS,
                        100.0);
                }
        });

        let black = black.read().build(ctx)?;
        let white = white.read().build(ctx)?;

        graphics::draw_ex(ctx, &black,
            DrawParam {
                color: Some(graphics::BLACK),
                .. Default::default()
            })?;

        graphics::draw_ex(ctx, &white,
            DrawParam {
                color: Some(graphics::WHITE),
                .. Default::default()
            })?;

        let text = Text::new(ctx, &format!("{}", timer::get_fps(ctx) as u32), &Font::default_font()?)?;
        
        graphics::draw_ex(ctx, &text,
            DrawParam {
                color: Some(Color::from((0, 255, 0))),
                .. Default::default()
            })?;

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            Keycode::R => self.reset(),
            Keycode::A => {
                println!("Auto mode on");
                self.auto_mode(true);
            }
            Keycode::S => {
                println!("Auto mode off");
                self.auto_mode(false);
            }
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
    config.window_mode.vsync = false;

    let ctx = &mut Context::load_from_conf("Reversi", "Roughsketch", config).unwrap();

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("assets");
        ctx.filesystem.mount(&path, true);
    }

    let mut game = match MainState::new(ctx) {
        Ok(game) => game,
        Err(why) => {
            println!("Could not load MainState: {:?}", why);
            return;
        }
    };

    graphics::set_background_color(ctx, Color::from((59, 122, 87)));
    let result = event::run(ctx, &mut game);

    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
