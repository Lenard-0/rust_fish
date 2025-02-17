use crate::Piece;
use super::Move;


/// Moves piece from current_pos to new_pos
/// Returns taken piece if any
pub fn move_piece(
    moving: &Move,
    board: &mut Vec<Vec<Option<Piece>>>
) -> Option<Piece> {
    let taken_piece = board[moving.new_pos.0][moving.new_pos.1].clone();
    let piece = board[moving.current_pos.0][moving.current_pos.1].clone();
    board[moving.new_pos.0][moving.new_pos.1] = piece;
    board[moving.current_pos.0][moving.current_pos.1] = None;
    return taken_piece
}

pub fn unmove_piece(
    moving: &Move,
    board: &mut Vec<Vec<Option<Piece>>>,
    taken_piece: Option<Piece>,
) {
    let piece = board[moving.new_pos.0][moving.new_pos.1].clone();
    board[moving.current_pos.0][moving.current_pos.1] = piece;
    board[moving.new_pos.0][moving.new_pos.1] = taken_piece;
}