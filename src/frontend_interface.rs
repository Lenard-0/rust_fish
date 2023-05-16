

pub fn emoji_piece_is_white(piece: &str) -> Result<bool, String> {
    return match piece {
        "♖"|"♘"|"♗"|"♕"|"♔"|"♙" => Ok(true),
        "♜"|"♞"|"♝"|"♛"|"♚"|"♟" => Ok(false),
        string => Err(format!("Cannot determine if piece is white as not valid piece. Found: {}", string))
    }
}