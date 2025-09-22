use rand::seq::SliceRandom;
use rand::{rng};
use crate::board;
use crate::board::Board;

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceType {
    fn shape(&self) -> Vec<(i32, i32)> {
        match self {
            PieceType::I => vec![(-2,0), (-1,0), (0,0), (1,0)],
            PieceType::O => vec![(0,0), (1,0), (0,1), (1,1)],
            PieceType::T => vec![(-1,0), (0,0), (1,0), (0,-1)],
            PieceType::S => vec![(0,0), (1,0), (0,-1), (-1,-1)],
            PieceType::Z => vec![(0,0), (-1,0), (0,-1), (1,-1)],
            PieceType::J => vec![(-1,0), (0,0), (1,0), (1,-1)],
            PieceType::L => vec![(-1,0), (0,0), (1,0), (-1,-1)],
        }
    }
}

pub struct Piece {
    piece_type: PieceType,
    pub(crate) shape: Vec<(i32, i32)>, // current shape of the piece, can move/rotate
    pub(crate) x: i32,                 // global position on the board
    pub(crate) y: i32,
}

pub fn new_piece(piece_type: PieceType) -> Piece {
    Piece {
        piece_type,
        shape: piece_type.shape(),
        x: 5,
        y: -2,
    }
}

pub fn move_down(piece: &mut Piece) {
    piece.y += 1;
}

pub fn check_move_down_allowed(piece: &Piece, board: &Board) -> bool {
    for (dx, dy) in &piece.shape {
        let tmp_dy = dy + piece.y + 1;
        let next_dy =  if tmp_dy.is_positive() { tmp_dy as usize } else { 0 };
        let v_dx = (dx + piece.x) as usize;

        // out of bound bottom
        if next_dy >= board::HEIGHT {
            return false;
        }

        // // x/new_y is already taken
        if board.cells[next_dy][v_dx].is_some() {
            return false
        }
    }

    true
}

pub fn move_lat(piece: &mut Piece, x: i32) {
    piece.x += x;
}

pub fn check_move_lat_allowed(piece: &Piece, board: &Board, move_x:i32) -> bool {
    // for each shape point, check if the move collides or out of bound
    for (dx, dy) in &piece.shape {
        let v_dy = dy + piece.y;

        let tmp_dx = dx + piece.x + move_x;
        let v_dx = if tmp_dx.is_positive() { tmp_dx as usize } else { 0 };
        // out of bound left/right
        if v_dx >= board::WIDTH || tmp_dx.is_negative() {
            return false
        }

        // // x/new_y is already taken
        if v_dy >= 0 && board.cells[v_dy as usize][v_dx].is_some() {
            return false
        }
    }

    true
}

pub fn rotate(piece: &mut Piece) {
    // for each shape point, we rotate 90deg
    // (x, y) becomes (-y, x)
    // very easy rotation
    for (dx, dy) in &mut piece.shape {
        let new_dx = -(*dy);
        let new_dy = *dx;
        *dx = new_dx;
        *dy = new_dy;
    }
}

pub fn check_rotate_allowed(piece: &Piece, board: &Board) -> bool {
    // TODO fix this function, can't rotate at the start of the new piece
    // this O piece shouldn't rotate, moving the whole block when shouldn't
    if piece.piece_type == PieceType::O {
        return false
    }

    // for each sahe point, check if the new rotation coord collides or out of bound
    for (dx, dy) in &piece.shape {
        let new_dx = -(*dy);
        let new_dy = *dx;
        let v_dx = (new_dx + piece.x) as usize;
        let v_dy = (new_dy + piece.y) as usize;

        // out of bound left/right
        if v_dx >= board::WIDTH {
            return false
        }

        // out of bound bottom
        if v_dy >= board::HEIGHT {
            return false
        }

        // new_x/new_y is already taken
        if board.cells[v_dy][v_dx].is_some() {
            return false
        }
    }

    true
}

pub fn generate_bag() -> Vec<PieceType> {
    let mut bag = vec![
        PieceType::I,
        PieceType::O,
        PieceType::T,
        PieceType::S,
        PieceType::Z,
        PieceType::J,
        PieceType::L,
    ];
    bag.shuffle(&mut rng());
    bag
}
