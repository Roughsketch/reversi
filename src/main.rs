use ggez::{Context, GameResult};
use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod, MouseButton};
use ggez::graphics::{self, Color, DrawMode, Point2, Rect};

const SCALE: f32 = 4.0;
const BOARD_SIZE: usize = 4;
const SPACE_SIZE: f32 = 30.0 * SCALE;
const RADIUS: f32 = SPACE_SIZE / 3.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Piece {
    Black,
    White,
}

struct MainState {
    board: [Option<Piece>; BOARD_SIZE * BOARD_SIZE],
    turn: Piece,
    winner: Option<Piece>,
}

impl MainState {
    pub fn new() -> GameResult<Self> {
        let mut board = [None; BOARD_SIZE * BOARD_SIZE];

        let idx = BOARD_SIZE / 2;

        board[BOARD_SIZE * (idx - 1) + idx - 1] = Some(Piece::White);
        board[BOARD_SIZE * (idx - 1) + idx] = Some(Piece::Black);
        board[(BOARD_SIZE * idx) + idx - 1] = Some(Piece::Black);
        board[(BOARD_SIZE * idx) + idx] = Some(Piece::White);
        Ok(Self {
            board,
            turn: Piece::White,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        
        let mut color_flag = false;

        for (index, piece) in self.board.iter().enumerate() {
            let col = (index % BOARD_SIZE) as f32;
            let row = (index / BOARD_SIZE) as f32;

            if col == 0.0 {
                color_flag = !color_flag;
            }

            if color_flag {
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

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            Keycode::A => self.board[5] = Some(Piece::White),
            _ => {}
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        if button == MouseButton::Left {
            let pos_x = (x as f32 / SPACE_SIZE) as usize;
            let pos_y = (y as f32 / SPACE_SIZE) as usize;
            let index = BOARD_SIZE * pos_y + pos_x;

            println!("Pos: {} {} - {}", pos_x, pos_y, index);

            self.board[index] = Some(self.turn);

            if self.turn == Piece::White {
                self.turn = Piece::Black;
            } else {
                self.turn = Piece::White;
            }
        }
    }
}

fn main() {
    let size = (SPACE_SIZE as usize * BOARD_SIZE) as u32;
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
