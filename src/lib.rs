use frontend_interface::emoji_piece_is_white;

pub mod frontend_interface;

pub enum Piece {
    Black(PieceType),
    White(PieceType),
}

impl Piece {
    pub fn from_emoji(piece: String) -> Result<Self, String> {
        return match emoji_piece_is_white(&piece) {
            Ok(true) => Ok(Self::White(PieceType::from_emoji(&piece))),
            Ok(false) => Ok(Self::Black(PieceType::from_emoji(&piece))),
            Err(err) => Err(err),
        }
    }
}

pub enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn
}

impl PieceType {
    // can safely return Self as have checked if valid piece in fn previously
    fn from_emoji(piece: &str) -> Self {
        return match piece {
            "♖"|"♜" => Self::Rook,
            "♘"|"♞" => Self::Knight,
            "♗"|"♝" => Self::Bishop,
            "♕"|"♛" => Self::Queen,
            "♔"|"♚" => Self::King,
            "♙"|"♟" => Self::Pawn,
            _ => unreachable!()
        }
    }
}

