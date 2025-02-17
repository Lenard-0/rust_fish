
use check::remove_moves_resulting_in_check;
use king::CastleState;
use crate::{Piece, PieceType};
use self::{pawn::calculate_pawn_moves, king::calculate_king_moves};
use crate::chess_functionality::moves::search_for_moves::search_for_moves;

pub mod pawn;
pub mod king;
pub mod search_for_moves;
pub mod check;
pub mod move_piece;

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub current_pos: (usize, usize),
    pub new_pos: (usize, usize),
    pub special_rule: Option<SpecialRule>
}

impl Move {
    pub fn from_position(
        current_pos: (usize, usize),
        new_pos: (usize, usize),
    ) -> Result<Self, String> {
        Ok(Self {
                current_pos,
                new_pos,
                special_rule: None,
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialRule {
    Enpassant,
    Castle,
    Promotion
}

pub fn calculate_possible_moves(
    ir: usize,
    ic: usize,
    board: &mut Vec<Vec<Option<Piece>>>,
    excluding_moves_that_result_in_check: bool,
    whites_turn: bool,
    previous_move: &Option<Move>,
    castle_state: &mut CastleState
) -> Result<Vec<Move>, String> {
    let possible_moves_before_excluding_check = get_possible_moves_before_excluding_check(board, ir, ic, whites_turn, previous_move, castle_state)?;
    if excluding_moves_that_result_in_check {
        return remove_moves_resulting_in_check(possible_moves_before_excluding_check, board, whites_turn, previous_move, castle_state)
    } else {
        return Ok(possible_moves_before_excluding_check)
    }
}

fn get_possible_moves_before_excluding_check(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
    previous_move: &Option<Move>,
    castle_state: &mut CastleState
) -> Result<Vec<Move>, String> {
    let piece_type = PieceType::from_piece(Piece::from_board(board, ir, ic, whites_turn)?);
    match piece_type {
        PieceType::Pawn => calculate_pawn_moves(board, ir, ic, whites_turn, previous_move),
        PieceType::King => calculate_king_moves(board, ir, ic, whites_turn, previous_move, castle_state),
        piece_type => search_for_moves(board, ir, ic, whites_turn, piece_type.directions(), piece_type.max_steps())
    }
}
