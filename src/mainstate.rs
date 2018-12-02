use ggez::GameResult;

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

pub struct MainState {
    pub board: [Option<Piece>; crate::RANK * crate::RANK],
    turn: Piece,
    winner: Option<Winner>,
}

impl MainState {
    pub fn new() -> GameResult<Self> {
        let mut board = [None; crate::RANK * crate::RANK];

        let idx = crate::RANK / 2;

        board[crate::RANK * (idx - 1) + idx - 1] = Some(Piece::White);
        board[crate::RANK * (idx - 1) + idx] = Some(Piece::Black);
        board[(crate::RANK * idx) + idx - 1] = Some(Piece::Black);
        board[(crate::RANK * idx) + idx] = Some(Piece::White);

        Ok(Self {
            board,
            turn: Piece::White,
            winner: None,
        })
    }

    pub fn reset(&mut self) {
        self.board = [None; crate::RANK * crate::RANK];
        self.turn = Piece::White;
        self.winner = None;

        let idx = crate::RANK / 2;

        self.board[crate::RANK * (idx - 1) + idx - 1] = Some(Piece::White);
        self.board[crate::RANK * (idx - 1) + idx] = Some(Piece::Black);
        self.board[(crate::RANK * idx) + idx - 1] = Some(Piece::Black);
        self.board[(crate::RANK * idx) + idx] = Some(Piece::White);
    }

    pub fn valid_space(&self, x: usize, y: usize) -> bool {
        let mut flipped = false;

        //  If spot isn't empty, then you can't place it
        if !self.board[crate::RANK * y + x].is_none() {
            return false;
        }

        //  Last spot left is always valid
        if self.board.into_iter().filter(|x| x.is_none()).count() == 1 {
            return true;
        }

        //  Check below
        for new_y in y + 1..crate::RANK {
            let new_idx = crate::RANK * new_y + x;

            if self.board[new_idx] == Some(self.turn) {
                if flipped {
                    return true;
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
            let new_idx = crate::RANK * new_y + x;

            if self.board[new_idx] == Some(self.turn) {
                if flipped {
                    return true;
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
            let new_idx = crate::RANK * y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                if flipped {
                    return true;
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
        for new_x in x + 1..crate::RANK {
            let new_idx = crate::RANK * y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                if flipped {
                    return true;
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
        for (new_x, new_y) in (x + 1..crate::RANK).zip(y + 1..crate::RANK) {
            let new_idx = crate::RANK * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                if flipped {
                    return true;
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
        for (new_x, new_y) in (x + 1..crate::RANK).zip((0..y).rev()) {
            let new_idx = crate::RANK * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                if flipped {
                    return true;
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
        for (new_x, new_y) in (0..x).rev().zip(y + 1..crate::RANK) {
            let new_idx = crate::RANK * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                if flipped {
                    return true;
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
            let new_idx = crate::RANK * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                if flipped {
                    return true;
                }
                break;
            } else if self.board[new_idx].is_none() {
                break;
            } else {
                flipped = true;
            }
        }

        false
    }

    pub fn captures(&self, x: usize, y: usize) -> Vec<usize> {
        //  List of positions to flip colors
        let mut captures = Vec::new();
        //  Storing potential flips while searching each direction
        let mut candidates = Vec::new();

        //  Check below
        for new_y in y + 1..crate::RANK {
            let new_idx = crate::RANK * new_y + x;

            if self.board[new_idx] == Some(self.turn) {
                captures.append(&mut candidates);
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
            let new_idx = crate::RANK * new_y + x;

            if self.board[new_idx] == Some(self.turn) {
                captures.append(&mut candidates);
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
            let new_idx = crate::RANK * y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                captures.append(&mut candidates);
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
        for new_x in x + 1..crate::RANK {
            let new_idx = crate::RANK * y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                captures.append(&mut candidates);
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
        for (new_x, new_y) in (x + 1..crate::RANK).zip(y + 1..crate::RANK) {
            let new_idx = crate::RANK * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                captures.append(&mut candidates);
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
        for (new_x, new_y) in (x + 1..crate::RANK).zip((0..y).rev()) {
            let new_idx = crate::RANK * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                captures.append(&mut candidates);
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
        for (new_x, new_y) in (0..x).rev().zip(y + 1..crate::RANK) {
            let new_idx = crate::RANK * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                captures.append(&mut candidates);
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
            let new_idx = crate::RANK * new_y + new_x;

            if self.board[new_idx] == Some(self.turn) {
                captures.append(&mut candidates);
                break;
            } else if self.board[new_idx].is_none() {
                candidates.clear();
                break;
            } else {
                candidates.push(new_idx);
            }
        }

        captures
    }

    pub fn place(&mut self, x: usize, y: usize) {
        self.board[crate::RANK * y + x] = Some(self.turn);

        for index in self.captures(x, y).iter() {
            self.board[*index] = Some(self.turn);
        }

        self.next_turn();
    }

    pub fn next_turn(&mut self) {
        if self.turn == Piece::White {
            self.turn = Piece::Black;
        } else {
            self.turn = Piece::White;
        }
    }

    pub fn has_move(&self) -> bool {
        for x in 0..crate::RANK {
            for y in 0..crate::RANK {
                if self.valid_space(x, y) {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_winner(&mut self) -> Winner {
        let white = self
            .board
            .iter()
            .filter(|&x| *x == Some(Piece::White))
            .count();
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
}
