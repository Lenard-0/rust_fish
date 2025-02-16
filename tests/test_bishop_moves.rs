
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState}, Piece, PieceType};

    #[test]
    fn bishop_valid_moves_middle_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Bishop)); // Bishop in the middle

        let mut expected_moves = vec![];
        for i in 1..8 {
            if 4 + i < 8 && 4 + i < 8 {
                expected_moves.push((4 + i, 4 + i)); // Down-right
            }
            if 4 + i < 8 && 4 >= i {
                expected_moves.push((4 + i, 4 - i)); // Down-left
            }
            if 4 >= i && 4 + i < 8 {
                expected_moves.push((4 - i, 4 + i)); // Up-right
            }
            if 4 >= i && 4 >= i {
                expected_moves.push((4 - i, 4 - i)); // Up-left
            }
        }

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn bishop_valid_moves_edge_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][4] = Some(Piece::White(PieceType::Bishop)); // Bishop on top edge

        let expected_moves = vec![
            (1, 5), (2, 6), (3, 7), // Down-right
            (1, 3), (2, 2), (3, 1), (4, 0) // Down-left
        ];

        let possible_moves = calculate_possible_moves(0, 4, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn bishop_valid_moves_corner_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::Bishop)); // Bishop in top-left corner

        let mut expected_moves = vec![];
        for i in 1..8 {
            expected_moves.push((i, i)); // Down-right diagonal moves
        }

        let possible_moves = calculate_possible_moves(0, 0, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn bishop_moves_blocked_by_same_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Bishop));
        board[6][6] = Some(Piece::White(PieceType::Pawn)); // Blocked diagonally

        let expected_moves = vec![
            (5, 5), // Blocked by same color bottom right
            (3, 3), (2, 2), (1, 1), (0, 0), // Up-left
            (5, 3), (6, 2), (7, 1), // Up-right
            (3, 5), (2, 6), (1, 7) // Down-left
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn bishop_moves_blocked_by_same_color_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::Bishop));
        board[6][6] = Some(Piece::Black(PieceType::Pawn)); // Blocked diagonally

        let expected_moves = vec![
            (5, 5), // Blocked by same color bottom right
            (3, 3), (2, 2), (1, 1), (0, 0), // Up-left
            (5, 3), (6, 2), (7, 1), // Up-right
            (3, 5), (2, 6), (1, 7) // Down-left
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn bishop_can_capture_opponent_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Bishop));
        board[6][6] = Some(Piece::Black(PieceType::Pawn)); // Capturable piece

        let expected_moves = vec![
            (5, 5), (6, 6), // Capture included bottom right
            (3, 3), (2, 2), (1, 1), (0, 0), // Up-left
            (5, 3), (6, 2), (7, 1), // Up-right
            (3, 5), (2, 6), (1, 7) // Down-left
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn bishop_can_capture_opponent_piece_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::Bishop));
        board[6][6] = Some(Piece::White(PieceType::Pawn)); // Capturable piece

        let expected_moves = vec![
            (5, 5), (6, 6), // Capture included bottom right
            (3, 3), (2, 2), (1, 1), (0, 0), // Up-left
            (5, 3), (6, 2), (7, 1), // Up-right
            (3, 5), (2, 6), (1, 7) // Down-left
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn bishop_cannot_move_outside_board() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::Bishop)); // Bishop in top-left corner

        let mut expected_moves = vec![];
        for i in 1..8 {
            expected_moves.push((i, i)); // Down-right diagonal moves
        }

        let possible_moves = calculate_possible_moves(0, 0, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn bishop_all_valid_moves_covered() {
        let mut board = vec![vec![None; 8]; 8];
        board[3][3] = Some(Piece::White(PieceType::Bishop)); // Bishop in the middle

        let mut expected_moves = vec![];
        for i in 1..8 {
            if 3 + i < 8 && 3 + i < 8 {
                expected_moves.push((3 + i, 3 + i)); // Down-right
            }
            if 3 + i < 8 && 3 >= i {
                expected_moves.push((3 + i, 3 - i)); // Down-left
            }
            if 3 >= i && 3 + i < 8 {
                expected_moves.push((3 - i, 3 + i)); // Up-right
            }
            if 3 >= i && 3 >= i {
                expected_moves.push((3 - i, 3 - i)); // Up-left
            }
        }

        let possible_moves = calculate_possible_moves(3, 3, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }
}