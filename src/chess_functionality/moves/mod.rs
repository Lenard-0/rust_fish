use crate::Piece;

use self::{rook::calculate_rook_moves, bishop::calculate_bishop_moves};

pub mod rook;
pub mod bishop;

pub fn calculate_possible_moves(
    ir: usize,
    ic: usize,
    board: &Vec<Vec<Option<Piece>>>
) -> Result<Vec<(usize, usize)>, String> {
    let piece = match &board[ir][ic] {
        Some(piece) => piece.clone(),
        None => return Err(format!("No piece at position: {} {}", ir, ic))
    };

    let whites_turn: bool;
    let piece_type = match piece {
        Piece::Black(piece) => {
            whites_turn = false;
            piece
        },
        Piece::White(piece) => {
            whites_turn = true;
            piece
        },
    };

    return Ok(match piece_type {
        crate::PieceType::Rook => calculate_rook_moves(board, ir, ic, whites_turn),
        crate::PieceType::Knight => return Err("Not yet completed".to_string()),
        crate::PieceType::Bishop => calculate_bishop_moves(board, ir, ic, whites_turn),
        crate::PieceType::Queen => vec![
            calculate_rook_moves(board, ir, ic, whites_turn),
            calculate_bishop_moves(board, ir, ic, whites_turn)
        ].concat(),
        crate::PieceType::King => return Err("Not yet completed".to_string()),
        crate::PieceType::Pawn => return Err("Not yet completed".to_string()),
    })
}