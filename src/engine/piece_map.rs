
// The point of this is to give slight evaluation bumps to squares that a piece generally
// prefers to sit on. For example, a knight prefers to sit in the center of the board



impl PieceType {
    pub fn get_piece_square_table_value(&self, row: usize, col: usize) -> i32 {
        match self {
            PieceType::Pawn => {
                return PAWN_TABLE[row][col]
            },
            PieceType::Knight => {
                return KNIGHT_TABLE[row][col]
            },
            PieceType::Bishop => {
                return BISHOP_TABLE[row][col]
            },
            PieceType::Rook => {
                return ROOK_TABLE[row][col]
            },
            PieceType::Queen => {
                return QUEEN_TABLE[row][col]
            },
            PieceType::King => {
                return KING_TABLE[row][col]
            }
        }
    }
}