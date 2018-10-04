use ggez::{Context, GameResult};
use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Point2, Rect};
use rayon::prelude::*;

/// The target window size width/height wise.
const WINDOW_SIZE: f32 = 800.0;
/// The rank of the board, or how many squares make up a side.
const RANK: usize = 100;
/// The size of a single square on the board.
const SPACE_SIZE: f32 = WINDOW_SIZE / RANK as f32;
/// The radius of a player's piece.
const RADIUS: f32 = SPACE_SIZE / 3.0;

mod mainstate;
use self::mainstate::{MainState, Piece};

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.board.par_iter().any(Option::is_none) {
            if self.is_auto() {
                self.reset();
            }

            self.check_winner();
        } else if self.is_auto() {
            let mut rng = rand::thread_rng();

            let candidates = (0..RANK*RANK).into_par_iter().filter(|index| {
                self.valid_space(index % RANK, index / RANK)
            }).collect::<Vec<usize>>();

            let choice = rand::sample(&mut rng, candidates, 1);

            if choice.len() > 0 {
                let index = choice[0];
                self.place(index % RANK, index / RANK);
            } else {
                self.next_turn();

                //  If neither side has a valid move, then end the game
                if !self.has_move() {
                    self.reset();
                }
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
        let start = std::time::Instant::now();

        graphics::clear(ctx);
        
        let mut color_flag = false;
        //  Keep track of which index has the spot with the most captures
        let mut best_spot = None;

        for col in 0..RANK {
            for row in 0..RANK {
                if col == 0 {
                    color_flag = !color_flag;
                }

                let rect_color = if self.valid_space(col, row) {
                    let total = self.captures(col, row).len();
                    if best_spot.is_none() || total > best_spot.unwrap_or(0) {
                        best_spot = Some(row * RANK + col);
                    }
                    Some(Color::from((255, 19, 22)))
                } else if color_flag {
                    Some(Color::from((158, 19, 22)))
                } else {
                    None
                };

                color_flag = !color_flag;

                if rect_color.is_some() {
                    graphics::draw_ex(ctx, &self.rect,
                        DrawParam {
                            dest: Point2::new(col as f32 * SPACE_SIZE, row as f32 * SPACE_SIZE),
                            color: rect_color,
                            .. Default::default()
                        })?;
                }
            }
        }
        println!("r{:?}", start.elapsed());

        for (index, piece) in self.board.iter().enumerate() {
            let col = (index % RANK) as f32;
            let row = (index / RANK) as f32;

            if let Some(p) = piece {
                let player_color = if *p == Piece::Black {
                    graphics::BLACK
                } else {
                    graphics::WHITE
                };

                graphics::draw_ex(ctx, &self.circle,
                    DrawParam {
                        dest: Point2::new(col * SPACE_SIZE + SPACE_SIZE / 2.0, row * SPACE_SIZE + SPACE_SIZE / 2.0),
                        color: Some(player_color),
                        .. Default::default()
                    })?;
                        
            }
        }
        
        if let Some(best) = best_spot {
            graphics::draw_ex(ctx, &self.rect,
                DrawParam {
                    dest: Point2::new((best % RANK) as f32 * SPACE_SIZE, (best / RANK) as f32 * SPACE_SIZE),
                    color: Some(Color::from((0, 255, 0))),
                    .. Default::default()
                })?;
        }

        graphics::present(ctx);

        println!("{:?}", start.elapsed());
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
