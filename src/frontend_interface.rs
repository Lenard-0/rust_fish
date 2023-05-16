use crate::Piece;



pub fn emoji_piece_is_white(piece: &str) -> Option<bool> {
    return match piece {
        "♖"|"♘"|"♗"|"♕"|"♔"|"♙" => Some(true),
        "♜"|"♞"|"♝"|"♛"|"♚"|"♟" => Some(false),
        _ => None
    }
}

pub fn parse_board(board: Vec<Vec<String>>) -> Vec<Vec<Option<Piece>>> {
    return board
        .iter()
        .map(|row| row
            .iter()
            .map(|tile| Piece::from_emoji(tile)).collect()
        ).collect()
}