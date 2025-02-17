use crate::{chess_functionality::moves::Move, Piece};


pub struct MoveThread {
    pub moves: Vec<Move>,
    pub current_score: i32,
    pub current_board: Vec<Vec<Option<Piece>>>
}