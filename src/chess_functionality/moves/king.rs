use crate::Piece;
use super::{search_for_moves::search_for_moves, Move};

pub fn calculate_king_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Result<Vec<Move>, String> {
    search_for_moves(board, ir, ic, whites_turn, &[(0,1), (0,-1), (1,0), (-1,0), (1,1), (1,-1), (-1,1), (-1,-1)], 1)
}