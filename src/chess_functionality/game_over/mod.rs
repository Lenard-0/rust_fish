use crate::Piece;
use super::moves::{check::{all_possible_moves, king_is_checked}, move_piece, unmove_piece, Move};


pub fn is_checkmate(board: &mut Vec<Vec<Option<Piece>>>, whites_turn: bool, previous_move: &Option<Move>) -> Result<bool, String> {
    if !king_is_checked(board, whites_turn, previous_move)? {
        return Ok(false)
    }

    return all_moves_result_in_check(board, whites_turn, previous_move)
}

pub fn is_stalemate(board: &mut Vec<Vec<Option<Piece>>>, whites_turn: bool, previous_move: &Option<Move>) -> Result<bool, String> {
    if king_is_checked(board, whites_turn, previous_move)? {
        return Ok(false)
    }

    return all_moves_result_in_check(board, whites_turn, previous_move)
}

fn all_moves_result_in_check(board: &mut Vec<Vec<Option<Piece>>>, whites_turn: bool, previous_move: &Option<Move>) -> Result<bool, String> {
    let possible_moves = all_possible_moves(board, whites_turn, previous_move)?;
    for m in possible_moves {
        let taken_piece = move_piece(&m, board);
        let check = king_is_checked(board, whites_turn, previous_move)?;
        unmove_piece(&m, board, taken_piece);
        if !check {
            return Ok(false)
        }
    }

    return Ok(true)
}