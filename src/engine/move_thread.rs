use crate::chess_functionality::moves::Move;


#[derive(Debug, Clone)]
pub struct EngineCalculation {
    pub best_move: Option<Move>,
    pub score: i32,
    // pub move_sequence: Vec<Move>,
}