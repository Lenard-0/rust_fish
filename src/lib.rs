
pub mod chess_functionality;

#[derive(Clone, Debug)]
pub enum Piece {
    Black(PieceType),
    White(PieceType),
}

#[derive(Clone, Debug)]
pub enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn
}
