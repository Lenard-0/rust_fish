use crate::Piece;
use super::{Move, SpecialRule};


/// Moves piece from current_pos to new_pos
/// Returns taken piece if any
pub fn move_piece(
    moving: &Move,
    board: &mut Vec<Vec<Option<Piece>>>
) -> Option<Piece> {
    let mut taken_piece = board[moving.new_pos.0][moving.new_pos.1].clone();
    let piece = board[moving.current_pos.0][moving.current_pos.1].clone();
    board[moving.new_pos.0][moving.new_pos.1] = piece;
    board[moving.current_pos.0][moving.current_pos.1] = None;

    match moving.special_rule {
        Some(SpecialRule::Enpassant) => {
            taken_piece = board[moving.current_pos.0][moving.new_pos.1].clone();
            board[moving.current_pos.0][moving.new_pos.1] = None;
        },
        Some(SpecialRule::Castle) => {
            if moving.new_pos.1 == 2 {
                let rook = board[moving.new_pos.0][0].clone();
                board[moving.new_pos.0][3] = rook;
                board[moving.new_pos.0][0] = None;
            } else if moving.new_pos.1 == 6 {
                let rook = board[moving.new_pos.0][7].clone();
                board[moving.new_pos.0][5] = rook;
                board[moving.new_pos.0][7] = None;
            }
        },
        _ => {}
    }
    return taken_piece
}

pub fn unmove_piece(
    moving: &Move,
    board: &mut Vec<Vec<Option<Piece>>>,
    taken_piece: Option<Piece>,
) {
    let piece = board[moving.new_pos.0][moving.new_pos.1].clone();
    board[moving.current_pos.0][moving.current_pos.1] = piece;

    match moving.special_rule {
        Some(SpecialRule::Enpassant) => {
            board[moving.current_pos.0][moving.new_pos.1] = taken_piece;
        },
        Some(SpecialRule::Castle) => {
            if moving.new_pos.1 == 2 {
                let rook = board[moving.new_pos.0][3].clone();
                board[moving.new_pos.0][0] = rook;
                board[moving.new_pos.0][3] = None;
            } else if moving.new_pos.1 == 6 {
                let rook = board[moving.new_pos.0][5].clone();
                board[moving.new_pos.0][7] = rook;
                board[moving.new_pos.0][5] = None;
            }
        },
        _ => board[moving.new_pos.0][moving.new_pos.1] = taken_piece
    }
}