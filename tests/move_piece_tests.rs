
#[cfg(test)]
mod tests {

    use rust_fish_chess_engine::{chess_functionality::moves::{king::CastleState, move_piece::{move_piece, unmove_piece}, Move, SpecialRule}, Piece, PieceType};
    #[test]
    fn can_move_piece_correctly() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][4] = Some(Piece::Black(PieceType::Queen));
        board[7][4] = Some(Piece::White(PieceType::Rook));

        let new_move = Move { current_pos: (6, 4), new_pos: (7, 4), special_rule: None };
        let taken_piece = move_piece(&new_move, &mut board);
        if let Some(Piece::Black(PieceType::Queen)) = board[7][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Queen)) = board[7][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::White(PieceType::Rook)) = taken_piece {
            assert!(true);
        } else {
            assert!(false);
        }

        unmove_piece(&new_move, &mut board, taken_piece);

        if let Some(Piece::White(PieceType::Rook)) = board[7][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Queen)) = board[6][4] {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn can_move_enpassant_correctly() {
        let mut board = vec![vec![None; 8]; 8];
        board[3][4] = Some(Piece::White(PieceType::Pawn));
        board[3][5] = Some(Piece::Black(PieceType::Pawn));

        let new_move = Move { current_pos: (3, 4), new_pos: (2, 5), special_rule: Some(SpecialRule::Enpassant) };
        let taken_piece = move_piece(&new_move, &mut board);
        if let Some(Piece::White(PieceType::Pawn)) = board[2][5] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Pawn)) = taken_piece {
            assert!(true);
        } else {
            assert!(false);
        }

        unmove_piece(&new_move, &mut board, taken_piece);

        if let Some(Piece::White(PieceType::Pawn)) = board[3][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Pawn)) = board[3][5] {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}