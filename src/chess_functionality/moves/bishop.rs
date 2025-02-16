use std::f32::INFINITY;
use crate::Piece;
use super::{search_for_moves::search_for_moves, Move};

pub fn calculate_bishop_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Result<Vec<Move>, String> {
    search_for_moves(board, ir, ic, whites_turn, &[(1,1), (1,-1), (-1,1), (-1,-1)], INFINITY as usize)
}