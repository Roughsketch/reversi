use ggez::{Context, GameResult};
use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod, MouseButton};
use ggez::graphics::{self, Color, DrawMode, Point2, Rect};

const WINDOW_SIZE: f32 = 800.0;
const SCALE: f32 = 4.0;
const BOARD_SIZE: usize = 10;
const SPACE_SIZE: f32 = WINDOW_SIZE / BOARD_SIZE as f32;
const RADIUS: f32 = SPACE_SIZE / 3.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Piece {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Winner {
    Black,
    White,
    Tie,
}

struct MainState {
    board: [Option<Piece>; BOARD_SIZE * BOARD_SIZE],
    turn: Piece,
    winner: Option<Winner>,
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
            winner: None,
        })
    }

    fn valid_for(&self, player: Piece, x: usize, y: usize) -> bool {
        let mut flipped = false;

        //  If spot isn't empty, then you can't place it
        if !self.board[BOARD_SIZE * y + x].is_none() {
            return false;
        }

        //  Last spot left is always valid
        if self.board.into_iter().filter(|x| x.is_none()).count() == 1 {
            return true;
        }

        //  Check below
        for new_y in (y + 1..BOARD_SIZE) {
            let new_idx = BOARD_SIZE * new_y + x;

            if self.board[new_idx] == Some(player) {
                if flipped {
                    return true
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }
        flipped = false;

        //  Check above
        for new_y in (0..y).rev() {
            let new_idx = BOARD_SIZE * new_y + x;

            if self.board[new_idx] == Some(player) {
                if flipped {
                    return true
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }
        flipped = false;

        //  Check left
        for new_x in (0..x).rev() {
            let new_idx = BOARD_SIZE * y + new_x;

            if self.board[new_idx] == Some(player) {
                if flipped {
                    return true
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }
        flipped = false;

        //  Check right
        for new_x in (x + 1..BOARD_SIZE) {
            let new_idx = BOARD_SIZE * y + new_x;

            if self.board[new_idx] == Some(player) {
                if flipped {
                    return true
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }
        flipped = false;

        //  Check diagonal down right
        for (new_x, new_y) in (x + 1..BOARD_SIZE).zip(y + 1..BOARD_SIZE) {
            let new_idx = BOARD_SIZE * new_y + new_x;

            if self.board[new_idx] == Some(player) {
                if flipped {
                    return true
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }
        flipped = false;

        //  Check diagonal up right
        for (new_x, new_y) in (x + 1..BOARD_SIZE).zip((0..y).rev()) {
            let new_idx = BOARD_SIZE * new_y + new_x;

            if self.board[new_idx] == Some(player) {
                if flipped {
                    return true
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }
        flipped = false;

        //  Check diagonal down right
        for (new_x, new_y) in (0..x).rev().zip(y + 1..BOARD_SIZE) {
            let new_idx = BOARD_SIZE * new_y + new_x;

            if self.board[new_idx] == Some(player) {
                if flipped {
                    return true
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }
        flipped = false;

        //  Check diagonal up right
        for (new_x, new_y) in (0..x).rev().zip((0..y).rev()) {
            let new_idx = BOARD_SIZE * new_y + new_x;

            if self.board[new_idx] == Some(player) {
                if flipped {
                    return true
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }
        flipped = false;

        false
    }

    fn place(&mut self, x: usize, y: usize) {
        //  List of positions to flip colors
        let mut flips = Vec::new();
        //  Storing potential flips while searching each direction
        let mut candidates = Vec::new();

        let index = BOARD_SIZE * y + x;
        self.board[index] = Some(self.turn);

        //  Check below
        for new_y in (y + 1..BOARD_SIZE) {
            let new_idx = BOARD_SIZE * new_y + x;

            if self.board[new_idx] == Some(self.turn) {
                flips.append(&mut candidates);
                candidates.clear();
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }
        candidates.clear();

        //  Check above
        for new_y in (0..y).rev() {
            let new_idx = BOARD_SIZE * new_y + x;

            if self.board[new_idx] == Some(self.turn) {
                flips.append(&mut candidates);
                candidates.clear();
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }
        candidates.clear();

        //  Check left
        for new_x in (0..x).rev() {
            let new_idx = BOARD_SIZE * y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                flips.append(&mut candidates);
                candidates.clear();
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }
        candidates.clear();

        //  Check right
        for new_x in (x + 1..BOARD_SIZE) {
            let new_idx = BOARD_SIZE * y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                flips.append(&mut candidates);
                candidates.clear();
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }
        candidates.clear();

        //  Check diagonal down right
        for (new_x, new_y) in (x + 1..BOARD_SIZE).zip(y + 1..BOARD_SIZE) {
            let new_idx = BOARD_SIZE * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                flips.append(&mut candidates);
                candidates.clear();
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }
        candidates.clear();

        //  Check diagonal up right
        for (new_x, new_y) in (x + 1..BOARD_SIZE).zip((0..y).rev()) {
            let new_idx = BOARD_SIZE * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                flips.append(&mut candidates);
                candidates.clear();
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }
        candidates.clear();

        //  Check diagonal down right
        for (new_x, new_y) in (0..x).rev().zip(y + 1..BOARD_SIZE) {
            let new_idx = BOARD_SIZE * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                flips.append(&mut candidates);
                candidates.clear();
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }
        candidates.clear();

        //  Check diagonal up right
        for (new_x, new_y) in (0..x).rev().zip((0..y).rev()) {
            let new_idx = BOARD_SIZE * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                flips.append(&mut candidates);
                candidates.clear();
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }
        candidates.clear();

        for index in flips.iter() {
            self.board[*index] = Some(self.turn);
        }

        if self.turn == Piece::White {
            self.turn = Piece::Black;
        } else {
            self.turn = Piece::White;
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.board.iter().any(Option::is_none) {
            let white = self.board.iter().filter(|&x| *x == Some(Piece::White)).count();
            let black = BOARD_SIZE * BOARD_SIZE - white;

            if black > white {
                self.winner = Some(Winner::Black);
            } else if white > black {
                self.winner = Some(Winner::White);
            } else {
                self.winner = Some(Winner::Tie);
            }
        } else {

        }
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

            if self.valid_for(self.turn, col as usize, row as usize) {
                graphics::set_color(ctx, Color::from((255, 19, 22)))?;
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

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            _ => {}
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        if button == MouseButton::Left {
            let pos_x = (x as f32 / SPACE_SIZE) as usize;
            let pos_y = (y as f32 / SPACE_SIZE) as usize;

            if self.valid_for(self.turn, pos_x, pos_y) {
                self.place(pos_x, pos_y);
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
