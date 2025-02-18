use crate::{chess_functionality::moves::Move, Piece};


#[derive(Debug, Clone)]
pub struct MoveThread {
    pub moves: Vec<Move>,
    pub current_score: i32,
    pub current_board_encoded: String,
}