

#[cfg(test)]
mod tests {
    use rust_fish::{chess_functionality::moves::calculate_possible_moves, Piece, PieceType};

    #[test]
    fn knight_valid_moves_middle_board() {
        let mut board = vec![
            vec![Some(Piece::Black(PieceType::Rook)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Queen)), Some(Piece::Black(PieceType::King)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Rook))],
            vec![Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn))],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, Some(Piece::White(PieceType::Knight)), None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn))],
            vec![Some(Piece::White(PieceType::Rook)), Some(Piece::White(PieceType::Knight)), Some(Piece::White(PieceType::Bishop)), Some(Piece::White(PieceType::Queen)), Some(Piece::White(PieceType::King)), Some(Piece::White(PieceType::Bishop)), Some(Piece::White(PieceType::Knight)), Some(Piece::White(PieceType::Rook))],
        ];

        let possible_moves = calculate_possible_moves(4, 2, &mut board, true).unwrap();

        let expected_moves = vec![
            (2, 1), (2, 3), (3, 4), (5, 4), (5, 0), (3, 0)
        ];

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");

        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn knight_valid_moves_edge_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][1] = Some(Piece::White(PieceType::Knight));

        let possible_moves = calculate_possible_moves(0, 1, &mut board, true).unwrap();

        let expected_moves = vec![(2, 0), (2, 2), (1, 3)];

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");

        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn knight_valid_moves_corner_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::Knight));

        let possible_moves = calculate_possible_moves(0, 0, &mut board, true).unwrap();

        let expected_moves = vec![(1, 2), (2, 1)];

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");

        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn knight_moves_blocked_by_same_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[3][3] = Some(Piece::White(PieceType::Knight));
        board[1][2] = Some(Piece::White(PieceType::Pawn));
        board[1][4] = Some(Piece::White(PieceType::Pawn));

        let possible_moves = calculate_possible_moves(3, 3, &mut board, true).unwrap();

        let expected_moves = vec![(2, 5), (4, 5), (5, 4), (5, 2), (4, 1), (2, 1)];

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");

        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn knight_can_capture_opponent_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[3][3] = Some(Piece::White(PieceType::Knight));
        board[1][2] = Some(Piece::Black(PieceType::Pawn));
        board[1][4] = Some(Piece::Black(PieceType::Pawn));

        let possible_moves = calculate_possible_moves(3, 3, &mut board, true).unwrap();

        let expected_moves = vec![
            (1, 2), (1, 4), (2, 5), (4, 5),
            (5, 4), (5, 2), (4, 1), (2, 1)
        ];

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");

        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn knight_cannot_move_outside_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::Knight)); // Knight in the top-left corner

        let possible_moves = calculate_possible_moves(0, 0, &mut board, true).unwrap();

        let expected_moves = vec![(1, 2), (2, 1)]; // Only valid L-shaped moves within board limits

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");

        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }

        // Another test case: Knight on top edge, not in the corner
        board[0][4] = Some(Piece::White(PieceType::Knight));
        let possible_moves = calculate_possible_moves(0, 4, &mut board, true).unwrap();

        let expected_moves = vec![(1, 2), (1, 6), (2, 3), (2, 5)];

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");

        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn knight_all_valid_moves_covered() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Knight)); // Knight in the middle

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true).unwrap();

        let expected_moves = vec![
            (2, 3), (2, 5), (3, 2), (3, 6),
            (5, 2), (5, 6), (6, 3), (6, 5)
        ]; // All possible knight moves from this position

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");

        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }


}