use crate::{Piece, chess_functionality::moves::calculate_possible_moves};



pub fn emoji_piece_is_white(piece: &str) -> Option<bool> {
    return match piece {
        "♖"|"♘"|"♗"|"♕"|"♔"|"♙" => Some(true),
        "♜"|"♞"|"♝"|"♛"|"♚"|"♟" => Some(false),
        _ => None
    }
}

pub fn calculate_possible_moves_interface(
    ir: usize,
    ic: usize,
    board: Vec<Vec<String>>
) -> Result<Vec<(usize, usize)>, String> {
    let parsed_board = parse_board(board)?;
    return calculate_possible_moves(ir, ic, &parsed_board)
}

pub fn parse_board(board: Vec<Vec<String>>) -> Result<Vec<Vec<Option<Piece>>>, String> {
    let mut parsed_board = Vec::new();
    for row in board {
        let mut new_row = Vec::new();
        for tile in row {
            new_row.push(Piece::from_emoji(&tile)?);
        }
        parsed_board.push(new_row);
    }
    // return Err("parsing board worked".to_string())
    return Ok(parsed_board)
}