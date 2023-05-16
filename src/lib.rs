use frontend_interface::emoji_piece_is_white;

pub mod frontend_interface;
pub mod chess_functionality;

#[derive(Clone)]
pub enum Piece {
    Black(PieceType),
    White(PieceType),
}

impl Piece {
    pub fn from_emoji(piece: &str) -> Option<Self> {
        return match emoji_piece_is_white(&piece) {
            Some(true) => Some(Self::White(PieceType::from_emoji(&piece))),
            Some(false) => Some(Self::Black(PieceType::from_emoji(&piece))),
            None => None
        }
    }
}

#[derive(Clone)]
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

