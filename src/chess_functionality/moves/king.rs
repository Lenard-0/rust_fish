use crate::{Piece, PieceType};
use super::{check::king_is_checked, move_piece, search_for_moves::search_for_moves, Move};

pub struct CastleState {
    pub white_king_moved: bool,
    pub white_left_rook_moved: bool,
    pub white_right_rook_moved: bool,
    pub black_king_moved: bool,
    pub black_left_rook_moved: bool,
    pub black_right_rook_moved: bool,
}

impl CastleState {
    pub fn new() -> CastleState {
        CastleState {
            white_king_moved: false,
            white_left_rook_moved: false,
            white_right_rook_moved: false,
            black_king_moved: false,
            black_left_rook_moved: false,
            black_right_rook_moved: false,
        }
    }
}

pub fn calculate_king_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
    previous_move: &Option<Move>,
    castle_state: &mut CastleState,
) -> Result<Vec<Move>, String> {
    let possible_moves = search_for_moves(board, ir, ic, whites_turn, PieceType::King.directions(), 1)?;
    if king_is_checked(board, whites_turn, previous_move, castle_state)? {
        return Ok(possible_moves)
    }
    let possible_castling_moves = calculate_castling_moves(board, whites_turn, castle_state)?;
    Ok(vec![possible_moves, possible_castling_moves].concat())
}

fn calculate_castling_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    castle_state: &CastleState,
) -> Result<Vec<Move>, String> {
    let mut possible_moves = Vec::new();
    if whites_turn {
        if !castle_state.white_king_moved {
            if !castle_state.white_right_rook_moved && board[7][5].is_none() && board[7][6].is_none() {
                // check if each of those squares are not under attack

                // make sure rook is in fact there as it could have been taken
                match board[7][7] {
                    Some(Piece::White(PieceType::Rook)) => possible_moves.push(Move { current_pos: (7, 4), new_pos: (7, 6) }),
                    _ => {}
                };
            }
            if !castle_state.white_left_rook_moved && board[7][1].is_none() && board[7][2].is_none() && board[7][3].is_none() {
                // make sure rook is in fact there as it could have been taken
                match board[7][0] {
                    Some(Piece::White(PieceType::Rook)) => possible_moves.push(Move { current_pos: (7, 4), new_pos: (7, 2) }),
                    _ => {}
                };
            }
        }
    } else {
        if !castle_state.black_king_moved {
            if !castle_state.black_right_rook_moved && board[0][5].is_none() && board[0][6].is_none() {
                match board[0][7] {
                    Some(Piece::Black(PieceType::Rook)) => possible_moves.push(Move { current_pos: (0, 4), new_pos: (0, 6) }),
                    _ => {}
                };
            }
            if !castle_state.black_left_rook_moved && board[0][1].is_none() && board[0][2].is_none() && board[0][3].is_none() {
                match board[0][0] {
                    Some(Piece::Black(PieceType::Rook)) => possible_moves.push(Move { current_pos: (0, 4), new_pos: (0, 2) }),
                    _ => {}
                };
            }
        }
    }

    Ok(possible_moves)
}