use ggez::{Context, GameResult};
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Point2};
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Piece {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Winner {
    Black,
    White,
    Tie,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

pub struct MainState {
    pub board: [Option<Piece>; crate::RANK * crate::RANK],
    turn: Piece,
    pub turns: usize,
    winner: Option<Winner>,
    auto_mode: bool,
    pub circle: Mesh,
    pub rect: Mesh,
    pub grid: Mesh,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut board = [None; crate::RANK * crate::RANK];

        let idx = crate::RANK / 2;

        board[crate::RANK * (idx - 1) + idx - 1] = Some(Piece::White);
        board[crate::RANK * (idx - 1) + idx] = Some(Piece::Black);
        board[(crate::RANK * idx) + idx - 1] = Some(Piece::Black);
        board[(crate::RANK * idx) + idx] = Some(Piece::White);

        
        let circle = Mesh::new_circle(ctx,
            DrawMode::Fill,
            Point2::new(0.0, 0.0),
            crate::RADIUS,
            10.0)?;

        let rect = Mesh::new_polygon(ctx,
            DrawMode::Fill,
            &[
                Point2::new(0.0, 0.0),
                Point2::new(0.0, crate::SPACE_SIZE),
                Point2::new(crate::SPACE_SIZE, crate::SPACE_SIZE),
                Point2::new(crate::SPACE_SIZE, 0.0),
                Point2::new(0.0, 0.0),
            ])?;

        let mut grid = MeshBuilder::new();

        for row in 0..crate::RANK {
            for col in (row % 2..crate::RANK).step_by(2) {
                let c = col as f32 * crate::SPACE_SIZE;
                let r = row as f32 * crate::SPACE_SIZE;

                grid.polygon(DrawMode::Fill,
                    &[
                        Point2::new(c, r),
                        Point2::new(c, r + crate::SPACE_SIZE),
                        Point2::new(c + crate::SPACE_SIZE, r + crate::SPACE_SIZE),
                        Point2::new(c + crate::SPACE_SIZE, r),
                        Point2::new(c, r),
                    ]);
            }
        }

        let grid = grid.build(ctx)?;

        Ok(Self {
            board,
            turn: Piece::White,
            turns: 0,
            winner: None,
            auto_mode: false,
            circle,
            rect,
            grid,
        })
    }

    pub fn reset(&mut self) {
        self.board = [None; crate::RANK * crate::RANK];
        self.turn = Piece::White;
        self.turns = 0;
        self.winner = None;

        let idx = crate::RANK / 2;

        self.board[crate::RANK * (idx - 1) + idx - 1] = Some(Piece::White);
        self.board[crate::RANK * (idx - 1) + idx] = Some(Piece::Black);
        self.board[(crate::RANK * idx) + idx - 1] = Some(Piece::Black);
        self.board[(crate::RANK * idx) + idx] = Some(Piece::White);
    }

    pub fn valid_space(&self, x: usize, y: usize) -> bool {
        //  If spot isn't empty, then you can't place it
        if !self.board[crate::RANK * y + x].is_none() {
            return false;
        }

        //t spot left is always valid
        if self.turns == crate::RANK * crate::RANK - 1 {
            return true;
        }

        static DIRECTIONS: [Direction;  8] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];

        DIRECTIONS.into_par_iter().any(|dir| {
            check_valid(&self.board, self.turn, *dir, x, y)
        })
    }

    pub fn captures(&self, x: usize, y: usize) -> Vec<usize> {
        static DIRECTIONS: [Direction;  8] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];

        DIRECTIONS.into_par_iter().flat_map(|dir| {
            check_captures(&self.board, self.turn, *dir, x, y)
        }).collect::<Vec<usize>>()
    }

    pub fn place(&mut self, x: usize, y: usize) {
        self.board[crate::RANK * y + x] = Some(self.turn);
        
        self.captures(x, y).iter().for_each(|&index| {
            self.board[index] = Some(self.turn);
        });

        self.next_turn();
    }

    pub fn next_turn(&mut self) {
        if self.turn == Piece::White {
            self.turn = Piece::Black;
        } else {
            self.turn = Piece::White;
        }

        self.turns += 1;
    }

    pub fn has_move(&self) -> bool {
        (0..crate::RANK * crate::RANK).into_par_iter().any(|index| {
            self.valid_space(index % crate::RANK, index % crate::RANK)
        })
    }

    pub fn check_winner(&mut self) -> Winner {
        let white = self.board.par_iter().filter(|&x| *x == Some(Piece::White)).count();
        let black = crate::RANK * crate::RANK - white;

        if black > white {
            self.winner = Some(Winner::Black);
        } else if white > black {
            self.winner = Some(Winner::White);
        } else {
            self.winner = Some(Winner::Tie);
        }

        self.winner.unwrap()
    }

    pub fn auto_mode(&mut self, value: bool) {
        self.auto_mode = value;
    }

    pub fn is_auto(&self) -> bool {
        self.auto_mode
    }
}

fn check_captures(board: &[Option<Piece>], turn: Piece, dir: Direction, x: usize, y: usize) -> Vec<usize> {
    match dir {
        Direction::Up => check_up(board, turn, x, y),
        Direction::Down => check_down(board, turn, x, y),
        Direction::Left => check_left(board, turn, x, y),
        Direction::Right => check_right(board, turn, x, y),
        Direction::UpRight => check_up_right(board, turn, x, y),
        Direction::UpLeft => check_up_left(board, turn, x, y),
        Direction::DownRight => check_down_right(board, turn, x, y),
        Direction::DownLeft => check_down_left(board, turn, x, y),
    }
}

fn check_valid(board: &[Option<Piece>], turn: Piece, dir: Direction, x: usize, y: usize) -> bool {
    match dir {
        Direction::Up => valid_up(board, turn, x, y),
        Direction::Down => valid_down(board, turn, x, y),
        Direction::Left => valid_left(board, turn, x, y),
        Direction::Right => valid_right(board, turn, x, y),
        Direction::UpRight => valid_up_right(board, turn, x, y),
        Direction::UpLeft => valid_up_left(board, turn, x, y),
        Direction::DownRight => valid_down_right(board, turn, x, y),
        Direction::DownLeft => valid_down_left(board, turn, x, y),
    }
}

fn check_down(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> Vec<usize> {
    //  List of positions to flip colors
    let mut captures = Vec::new();
    //  Storing potential flips while searching each direction
    let mut candidates = Vec::new();

    //  Check below
    for new_y in y + 1..crate::RANK {
        let new_idx = crate::RANK * new_y + x;

        if board[new_idx] == Some(turn) {
            captures.append(&mut candidates);
            break;
        } else if board[new_idx].is_none() {
            candidates.clear();
            break;
        } else {
            candidates.push(new_idx);
        }
    }

    captures
}

fn check_up(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> Vec<usize> {
    //  List of positions to flip colors
    let mut captures = Vec::new();
    //  Storing potential flips while searching each direction
    let mut candidates = Vec::new();

    //  Check above
    for new_y in (0..y).rev() {
        let new_idx = crate::RANK * new_y + x;

        if board[new_idx] == Some(turn) {
            captures.append(&mut candidates);
            break;
        } else if board[new_idx].is_none() {
            candidates.clear();
            break;
        } else {
            candidates.push(new_idx);
        }
    }

    captures
}

fn check_left(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> Vec<usize> {
    //  List of positions to flip colors
    let mut captures = Vec::new();
    //  Storing potential flips while searching each direction
    let mut candidates = Vec::new();

    //  Check left
    for new_x in (0..x).rev() {
        let new_idx = crate::RANK * y + new_x;

        if board[new_idx] == Some(turn) {
            captures.append(&mut candidates);
            break;
        } else if board[new_idx].is_none() {
            candidates.clear();
            break;
        } else {
            candidates.push(new_idx);
        }
    }

    captures
}


fn check_right(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> Vec<usize> {
    //  List of positions to flip colors
    let mut captures = Vec::new();
    //  Storing potential flips while searching each direction
    let mut candidates = Vec::new();

    //  Check right
    for new_x in x + 1..crate::RANK {
        let new_idx = crate::RANK * y + new_x;

        if board[new_idx] == Some(turn) {
            captures.append(&mut candidates);
            break;
        } else if board[new_idx].is_none() {
            candidates.clear();
            break;
        } else {
            candidates.push(new_idx);
        }
    }

    captures
}

fn check_down_right(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> Vec<usize> {
    //  List of positions to flip colors
    let mut captures = Vec::new();
    //  Storing potential flips while searching each direction
    let mut candidates = Vec::new();

    //  Check diagonal down right
    for (new_x, new_y) in (x + 1..crate::RANK).zip(y + 1..crate::RANK) {
        let new_idx = crate::RANK * new_y + new_x;

        if board[new_idx] == Some(turn) {
            captures.append(&mut candidates);
            break;
        } else if board[new_idx].is_none() {
            candidates.clear();
            break;
        } else {
            candidates.push(new_idx);
        }
    }

    captures
}

fn check_up_right(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> Vec<usize> {
    //  List of positions to flip colors
    let mut captures = Vec::new();
    //  Storing potential flips while searching each direction
    let mut candidates = Vec::new();

    //  Check diagonal up right
    for (new_x, new_y) in (x + 1..crate::RANK).zip((0..y).rev()) {
        let new_idx = crate::RANK * new_y + new_x;

        if board[new_idx] == Some(turn) {
            captures.append(&mut candidates);
            break;
        } else if board[new_idx].is_none() {
            candidates.clear();
            break;
        } else {
            candidates.push(new_idx);
        }
    }

    captures
}

fn check_down_left(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> Vec<usize> {
    //  List of positions to flip colors
    let mut captures = Vec::new();
    //  Storing potential flips while searching each direction
    let mut candidates = Vec::new();

    //  Check diagonal down left
    for (new_x, new_y) in (0..x).rev().zip(y + 1..crate::RANK) {
        let new_idx = crate::RANK * new_y + new_x;

        if board[new_idx] == Some(turn) {
            captures.append(&mut candidates);
            break;
        } else if board[new_idx].is_none() {
            candidates.clear();
            break;
        } else {
            candidates.push(new_idx);
        }
    }

    captures
}

fn check_up_left(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> Vec<usize> {
    //  List of positions to flip colors
    let mut captures = Vec::new();
    //  Storing potential flips while searching each direction
    let mut candidates = Vec::new();

    //  Check diagonal up left
    for (new_x, new_y) in (0..x).rev().zip((0..y).rev()) {
        let new_idx = crate::RANK * new_y + new_x;

        if board[new_idx] == Some(turn) {
            captures.append(&mut candidates);
            break;
        } else if board[new_idx].is_none() {
            candidates.clear();
            break;
        } else {
            candidates.push(new_idx);
        }
    }

    captures
}

fn valid_down(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> bool {
    let mut flipped = false;

    //  Check below
    for new_y in y + 1..crate::RANK {
        let new_idx = crate::RANK * new_y + x;

        if board[new_idx] == Some(turn) {
            if flipped {
                return true
            }
            break;
        } else if board[new_idx].is_none() {
            return false
        } else {
            flipped = true;
        }
    }
    flipped
}
fn valid_up(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> bool {
    let mut flipped = false;

    //  Check above
    for new_y in (0..y).rev() {
        let new_idx = crate::RANK * new_y + x;

        if board[new_idx] == Some(turn) {
            if flipped {
                return true
            }
            break;
        } else if board[new_idx].is_none() {
            return false
        } else {
            flipped = true;
        }
    }
    flipped
}
fn valid_left(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> bool {
    let mut flipped = false;

    //  Check left
    for new_x in (0..x).rev() {
        let new_idx = crate::RANK * y + new_x;

        if board[new_idx] == Some(turn) {
            if flipped {
                return true
            }
            break;
        } else if board[new_idx].is_none() {
            return false
        } else {
            flipped = true;
        }
    }
    flipped
}

fn valid_right(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> bool {
    let mut flipped = false;

    //  Check right
    for new_x in x + 1..crate::RANK {
        let new_idx = crate::RANK * y + new_x;

        if board[new_idx] == Some(turn) {
            if flipped {
                return true
            }
            break;
        } else if board[new_idx].is_none() {
            return false
        } else {
            flipped = true;
        }
    }
    flipped
}

fn valid_down_right(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> bool {
    let mut flipped = false;

    //  Check diagonal down right
    for (new_x, new_y) in (x + 1..crate::RANK).zip(y + 1..crate::RANK) {
        let new_idx = crate::RANK * new_y + new_x;

        if board[new_idx] == Some(turn) {
            if flipped {
                return true
            }
            break;
        } else if board[new_idx].is_none() {
            return false
        } else {
            flipped = true;
        }
    }
    flipped
}

fn valid_up_right(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> bool {
    let mut flipped = false;

    //  Check diagonal up right
    for (new_x, new_y) in (x + 1..crate::RANK).zip((0..y).rev()) {
        let new_idx = crate::RANK * new_y + new_x;

        if board[new_idx] == Some(turn) {
            if flipped {
                return true
            }
            break;
        } else if board[new_idx].is_none() {
            return false
        } else {
            flipped = true;
        }
    }
    flipped
}

fn valid_down_left(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> bool {
    let mut flipped = false;

    //  Check diagonal down left
    for (new_x, new_y) in (0..x).rev().zip(y + 1..crate::RANK) {
        let new_idx = crate::RANK * new_y + new_x;

        if board[new_idx] == Some(turn) {
            if flipped {
                return true
            }
            break;
        } else if board[new_idx].is_none() {
            return false
        } else {
            flipped = true;
        }
    }
    flipped
}

fn valid_up_left(board: &[Option<Piece>], turn: Piece, x: usize, y: usize) -> bool {
    let mut flipped = false;

    //  Check diagonal up left
    for (new_x, new_y) in (0..x).rev().zip((0..y).rev()) {
        let new_idx = crate::RANK * new_y + new_x;

        if board[new_idx] == Some(turn) {
            if flipped {
                return true
            }
            break;
        } else if board[new_idx].is_none() {
            break;
        } else {
            flipped = true;
        }
    }
    flipped
}