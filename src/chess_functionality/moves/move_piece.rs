use crate::{Piece, PieceType};
use super::{king::CastleState, Move, SpecialRule};


/// Moves piece from current_pos to new_pos
/// Returns taken piece if any
pub fn move_piece(
    moving: &Move,
    board: &mut Vec<Vec<Option<Piece>>>,
    castle_state: &mut CastleState,
) -> Option<Piece> {
    update_castle_state(moving, castle_state);
    let mut taken_piece = board[moving.new_pos.0][moving.new_pos.1].clone();
    let piece = board[moving.current_pos.0][moving.current_pos.1].clone();
    board[moving.new_pos.0][moving.new_pos.1] = piece.clone();
    board[moving.current_pos.0][moving.current_pos.1] = None;

    match &moving.special_rule {
        Some(SpecialRule::Enpassant) => {
            taken_piece = board[moving.current_pos.0][moving.new_pos.1].clone();
            board[moving.current_pos.0][moving.new_pos.1] = None;
        },
        Some(SpecialRule::Castle) => {
            if moving.new_pos.1 == 2 {
                let rook = board[moving.new_pos.0][0].clone();
                board[moving.new_pos.0][3] = rook;
                board[moving.new_pos.0][0] = None;
            } else if moving.new_pos.1 == 6 {
                let rook = board[moving.new_pos.0][7].clone();
                board[moving.new_pos.0][5] = rook;
                board[moving.new_pos.0][7] = None;
            }
        },
        Some(SpecialRule::Promotion(piece_type)) => {
            board[moving.new_pos.0][moving.new_pos.1] = Some(match piece {
                Some(Piece::White(_)) => Piece::White(piece_type.clone()),
                Some(Piece::Black(_)) => Piece::Black(piece_type.clone()),
                _ => return None
            });
        },
        None => {}
    }
    return taken_piece
}

pub fn unmove_piece(
    moving: &Move,
    board: &mut Vec<Vec<Option<Piece>>>,
    taken_piece: Option<Piece>,
    castle_state: &mut CastleState,
) -> Result<(), String> {
    let piece = board[moving.new_pos.0][moving.new_pos.1].clone();
    unupdate_castle_state(moving, &piece, castle_state);
    board[moving.current_pos.0][moving.current_pos.1] = piece.clone();

    match moving.special_rule {
        Some(SpecialRule::Enpassant) => {
            board[moving.current_pos.0][moving.new_pos.1] = taken_piece;
        },
        Some(SpecialRule::Castle) => {
            if moving.new_pos.1 == 2 {
                let rook = board[moving.new_pos.0][3].clone();
                board[moving.new_pos.0][0] = rook;
                board[moving.new_pos.0][3] = None;
            } else if moving.new_pos.1 == 6 {
                let rook = board[moving.new_pos.0][5].clone();
                board[moving.new_pos.0][7] = rook;
                board[moving.new_pos.0][5] = None;
            }
        },
        Some(SpecialRule::Promotion(_)) => board[moving.new_pos.0][moving.new_pos.1] = match piece {
            Some(Piece::White(_)) => Some(Piece::White(PieceType::Pawn)),
            Some(Piece::Black(_)) => Some(Piece::Black(PieceType::Pawn)),
            None => return Err(format!("No piece to undo promote: {:?} {:?}", moving, board))
        },
        None => board[moving.new_pos.0][moving.new_pos.1] = taken_piece
    }

    Ok(())
}

fn update_castle_state(moving: &Move, castle_state: &mut CastleState) {
    match moving.current_pos {
        (0, 4) => castle_state.black_king_moved = true,
        (0, 0) => castle_state.black_left_rook_moved = true,
        (0, 7) => castle_state.black_right_rook_moved = true,
        (7, 4) => castle_state.white_king_moved = true,
        (7, 0) => castle_state.white_left_rook_moved = true,
        (7, 7) => castle_state.white_right_rook_moved = true,
        _ => {}
    }
}

fn unupdate_castle_state(moving: &Move, piece: &Option<Piece>, castle_state: &mut CastleState) {
    match moving.current_pos {
        (0, 4) => match piece {
            Some(Piece::Black(PieceType::King)) => castle_state.black_king_moved = false,
            _ => {}
        },
        (0, 0) => match piece {
            Some(Piece::Black(PieceType::Rook)) => castle_state.black_left_rook_moved = false,
            _ => {}
        },
        (0, 7) => match piece {
            Some(Piece::Black(PieceType::Rook)) => castle_state.black_right_rook_moved = false,
            _ => {}
        },
        (7, 4) => match piece {
            Some(Piece::White(PieceType::King)) => castle_state.white_king_moved = false,
            _ => {}
        },
        (7, 0) => match piece {
            Some(Piece::White(PieceType::Rook)) => castle_state.white_left_rook_moved = false,
            _ => {}
        },
        (7, 7) => match piece {
            Some(Piece::White(PieceType::Rook)) => castle_state.white_right_rook_moved = false,
            _ => {}
        },
        _ => {}
    }
}