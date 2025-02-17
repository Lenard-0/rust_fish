use std::f32::INFINITY;

use crate::{chess_functionality::game_over::is_checkmate, Piece};


pub fn determine_move_score(
    taken_piece: Option<Piece>
) -> Result<i32, String> {
    return Ok(0)
    // if is_checkmate(board, whites_turn, previous_move, castle_state)? {
    //     return Ok(INFINITY as i32);
    // }

    // match taken_piece {
    //     Some(Piece::Black(piece_type)) => Ok(match piece_type {
    //         PieceType::Rook => 5,
    //         PieceType::Knight => 3,
    //         PieceType::Bishop => 3,
    //         PieceType::Queen => 9,
    //         PieceType::King => 1000,
    //         PieceType::Pawn => 1
    //     }),
    //     Some(Piece::White(piece_type)) => Ok(match piece_type {
    //         PieceType::Rook => -5,
    //         PieceType::Knight => -3,
    //         PieceType::Bishop => -3,
    //         PieceType::Queen => -9,
    //         PieceType::King => -1000,
    //         PieceType::Pawn => -1
    //     }),
    //     None => Ok(0)
    // }

}