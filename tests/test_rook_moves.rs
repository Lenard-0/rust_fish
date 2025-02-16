
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState}, Piece, PieceType};

    #[test]
    fn rook_valid_moves_middle_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Rook)); // Rook in the middle

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None, &mut CastleState::new()).unwrap();

        let mut expected_moves = vec![];
        for i in 0..8 {
            if i != 4 {
                expected_moves.push((4, i)); // Horizontal moves
                expected_moves.push((i, 4)); // Vertical moves
            }
        }

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn rook_valid_moves_edge_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][4] = Some(Piece::White(PieceType::Rook)); // Rook on top edge

        let mut expected_moves = vec![];
        for i in 1..8 {
            expected_moves.push((i, 4)); // Downward vertical moves
        }
        for i in 0..8 {
            if i != 4 {
                expected_moves.push((0, i)); // Horizontal moves
            }
        }

        let possible_moves = calculate_possible_moves(0, 4, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn rook_valid_moves_corner_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::Rook)); // Rook in top-left corner

        let mut expected_moves = vec![];
        for i in 1..8 {
            expected_moves.push((i, 0)); // Vertical moves
            expected_moves.push((0, i)); // Horizontal moves
        }

        let possible_moves = calculate_possible_moves(0, 0, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn rook_moves_blocked_by_same_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Rook));
        board[4][6] = Some(Piece::White(PieceType::Pawn)); // Blocked horizontally
        board[6][4] = Some(Piece::White(PieceType::Pawn)); // Blocked vertically

        let expected_moves = vec![
            (4, 5), (4, 3), (4, 2), (4, 1), (4, 0),
            (5, 4), (3, 4), (2, 4), (1, 4), (0, 4)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn rook_moves_blocked_by_same_color_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::Rook));
        board[4][6] = Some(Piece::Black(PieceType::Pawn)); // Blocked horizontally
        board[6][4] = Some(Piece::Black(PieceType::Pawn)); // Blocked vertically

        let expected_moves = vec![
            (4, 5), (4, 3), (4, 2), (4, 1), (4, 0),
            (5, 4), (3, 4), (2, 4), (1, 4), (0, 4)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn rook_can_capture_opponent_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Rook));
        board[4][6] = Some(Piece::Black(PieceType::Pawn)); // Capturable piece
        board[6][4] = Some(Piece::Black(PieceType::Pawn)); // Capturable piece

        let expected_moves = vec![
            (4, 5), (4, 6), (4, 3), (4, 2), (4, 1), (4, 0),
            (5, 4), (6, 4), (3, 4), (2, 4), (1, 4), (0, 4)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn rook_can_capture_opponent_piece_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::Rook));
        board[4][6] = Some(Piece::White(PieceType::Pawn)); // Capturable piece
        board[6][4] = Some(Piece::White(PieceType::Pawn)); // Capturable piece

        let expected_moves = vec![
            (4, 5), (4, 6), (4, 3), (4, 2), (4, 1), (4, 0),
            (5, 4), (6, 4), (3, 4), (2, 4), (1, 4), (0, 4)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn rook_cannot_move_outside_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::Rook)); // Rook in top-left corner

        let mut expected_moves = vec![];
        for i in 1..8 {
            expected_moves.push((i, 0)); // Vertical moves
            expected_moves.push((0, i)); // Horizontal moves
        }

        let possible_moves = calculate_possible_moves(0, 0, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn rook_all_valid_moves_covered() {
        let mut board = vec![vec![None; 8]; 8];
        board[3][3] = Some(Piece::White(PieceType::Rook)); // Rook in the middle

        let mut expected_moves = vec![];
        for i in 0..8 {
            if i != 3 {
                expected_moves.push((3, i)); // Horizontal moves
                expected_moves.push((i, 3)); // Vertical moves
            }
        }

        let possible_moves = calculate_possible_moves(3, 3, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

}
