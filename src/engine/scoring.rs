
use crate::{utils::for_each_tile, Piece, PieceType};

pub const ROOK_VALUE: i32 = 500;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 310;
pub const QUEEN_VALUE: i32 = 900;
pub const PAWN_VALUE: i32 = 100;

pub fn evaluate_board(board: &Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<i32, String> {
    let white_eval = count_material(&board, true)?;
    let black_eval = count_material(&board, false)?;
    let eval = white_eval - black_eval;
    return match whites_turn {
        true => Ok(eval),
        false => Ok(eval * -1)
    }
}

fn count_material(board: &Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<i32, String> {
    let mut score = 0;
    for_each_tile(board, |_, _, tile| {
        match tile {
            Some(Piece::White(piece_type)) if whites_turn => score += piece_type.get_value(),
            Some(Piece::Black(piece_type)) if !whites_turn => score += piece_type.get_value(),
            _ => {}
        }
        Ok(())
    })?;
    return Ok(score)
}

impl PieceType {
    pub fn get_value(&self) -> i32 {
        match self {
            PieceType::Rook => ROOK_VALUE,
            PieceType::Knight => KNIGHT_VALUE,
            PieceType::Bishop => BISHOP_VALUE,
            PieceType::Queen => QUEEN_VALUE,
            PieceType::Pawn => PAWN_VALUE,
            PieceType::King => 0
        }
    }
}
// pub fn determine_move_score(
//     taken_piece: Option<Piece>,
//     players_turn: bool,
//     board: &mut Vec<Vec<Option<Piece>>>,
//     whites_turn: bool,
//     previous_move: &Option<Move>,
//     castle_state: &mut CastleState
// ) -> Result<i32, String> {
//     if is_checkmate(board, whites_turn, previous_move, castle_state)? {
//         match players_turn {
//             true => return Ok(INFINITY as i32),
//             false => return Ok(-INFINITY as i32)
//         }
//     }

//     let mut score = match taken_piece {
//         Some(Piece::Black(piece_type)) => match piece_type {
//             PieceType::Rook => 500,
//             PieceType::Knight => 300,
//             PieceType::Bishop => 300,
//             PieceType::Queen => 900,
//             PieceType::King => INFINITY as i32,
//             PieceType::Pawn => 100
//         },
//         Some(Piece::White(piece_type)) => match piece_type {
//             PieceType::Rook => 5,
//             PieceType::Knight => 3,
//             PieceType::Bishop => 3,
//             PieceType::Queen => 9,
//             PieceType::King => 10000000,
//             PieceType::Pawn => 1
//         },
//         None => 0
//     };

//     if !players_turn {
//         return Ok(-score)
//     } else {
//         return Ok(score)
//     }

// }