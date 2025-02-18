use std::f32::INFINITY;

use crate::{chess_functionality::{game_over::is_checkmate, moves::{king::CastleState, Move}}, Piece, PieceType};


pub fn determine_move_score(
    taken_piece: Option<Piece>,
    players_turn: bool,
    board: &mut Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    previous_move: &Option<Move>,
    castle_state: &mut CastleState
) -> Result<i32, String> {
    if is_checkmate(board, whites_turn, previous_move, castle_state)? {
        match players_turn {
            true => return Ok(INFINITY as i32),
            false => return Ok(-INFINITY as i32)
        }
    }

    let mut score = match taken_piece {
        Some(Piece::Black(piece_type)) => match piece_type {
            PieceType::Rook => 5,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Queen => 9,
            PieceType::King => 1000,
            PieceType::Pawn => 1
        },
        Some(Piece::White(piece_type)) => match piece_type {
            PieceType::Rook => 5,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Queen => 9,
            PieceType::King => 1000,
            PieceType::Pawn => 1
        },
        None => 0
    };

    if !players_turn {
        return Ok(-score)
    } else {
        return Ok(score)
    }

}