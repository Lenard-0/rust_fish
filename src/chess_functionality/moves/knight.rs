use crate::Piece;
use super::{search_for_moves::search_for_moves, Move};

pub fn calculate_knight_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Result<Vec<Move>, String>  {
    search_for_moves(board, ir, ic, whites_turn, &[(-2,-1), (-2, 1), (2,-1), (2, 1), (-1, 2), (1, 2), (-1, -2), (1, -2)], 1)
}