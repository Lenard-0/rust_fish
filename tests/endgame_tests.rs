
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::{game_over::{is_checkmate, is_stalemate}, moves::{calculate_possible_moves, king::CastleState}}, Piece, PieceType};

    #[test]
    fn checkmate_scenario() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[6][5] = Some(Piece::Black(PieceType::Queen));
        board[5][7] = Some(Piece::Black(PieceType::Rook));

        let possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), 0, "King should have no legal moves - Checkmate!");

        assert!(!is_stalemate(&mut board, true, &None, &mut CastleState::new()).unwrap());
        assert!(is_checkmate(&mut board, true, &None, &mut CastleState::new()).unwrap());
    }

    #[test]
    fn stalemate_scenario() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[5][6] = Some(Piece::Black(PieceType::Queen));

        let possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), 0, "King should have no legal moves - Stalemate!");

        assert!(!is_checkmate(&mut board, true, &None, &mut CastleState::new()).unwrap());
        assert!(is_stalemate(&mut board, true, &None, &mut CastleState::new()).unwrap());
    }
}