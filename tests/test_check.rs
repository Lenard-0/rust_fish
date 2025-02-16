
#[cfg(test)]
mod tests {
    use rust_fish::{chess_functionality::moves::calculate_possible_moves, Piece, PieceType};

    #[test]
    fn cannot_move_into_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[6][4] = Some(Piece::Black(PieceType::Queen));

        let expected_moves = vec![
            (7, 6)
        ];
        let possible_moves = calculate_possible_moves(7, 7, &mut board, true, true).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn cannot_move_piece_blocking_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));
        board[4][5] = Some(Piece::White(PieceType::Pawn)); // Blocker
        board[3][4] = Some(Piece::Black(PieceType::Queen)); // Black piece attacking the king

        // Try to move the pawn (blocking the check)
        let possible_moves = calculate_possible_moves(4, 5, &mut board, true, true).unwrap();
        assert!(possible_moves.is_empty(), "Blocked piece should not move while blocking check");
    }

    #[test]
    fn can_avoid_check_by_moving_king() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));
        board[3][4] = Some(Piece::Black(PieceType::Queen)); // Black queen attacking the king

        // The king can move to (5, 4) to avoid check
        let expected_moves = vec![(5, 4)];
        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn can_avoid_check_by_blocking_with_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));
        board[3][4] = Some(Piece::Black(PieceType::Queen)); // Black queen attacking the king
        board[5][4] = Some(Piece::White(PieceType::Rook)); // Blocker

        // The rook can block the check
        let expected_moves = vec![(5, 4)];
        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn cannot_move_piece_when_king_is_in_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));
        board[3][4] = Some(Piece::Black(PieceType::Queen)); // Black queen attacking the king
        board[5][5] = Some(Piece::White(PieceType::Pawn)); // White pawn to be moved

        // White pawn cannot move while the king is in check
        let possible_moves = calculate_possible_moves(5, 5, &mut board, true, true).unwrap();
        assert!(possible_moves.is_empty(), "Piece should not move when king is in check");
    }

    #[test]
    fn can_move_piece_that_does_not_result_in_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));
        board[3][4] = Some(Piece::Black(PieceType::Queen)); // Black queen attacking the king
        board[5][5] = Some(Piece::White(PieceType::Rook)); // White pawn to be moved

        // Move the white pawn (no checkmate)
        let expected_moves = vec![(5, 6)]; // Example of a move
        let possible_moves = calculate_possible_moves(5, 5, &mut board, true, true).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn cannot_move_king_into_check2() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));
        board[3][4] = Some(Piece::Black(PieceType::Queen)); // Black queen attacking the king
        board[5][4] = Some(Piece::Black(PieceType::Rook));  // Black rook creating checkmate

        // King cannot move into a checkmate position
        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true).unwrap();
        assert!(possible_moves.is_empty(), "King cannot move into checkmate");
    }
}