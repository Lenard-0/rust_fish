
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState, Move}, Piece, PieceType};

    #[test]
    fn white_pawn_moves_one_or_two_steps_forward_from_start() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][0] = Some(Piece::White(PieceType::Pawn));

        let expected_moves = vec![(5, 0), (4, 0)];

        let possible_moves = calculate_possible_moves(6, 0, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn white_pawn_moves_one_step_not_starting() {
        let mut board = vec![vec![None; 8]; 8];
        board[5][0] = Some(Piece::White(PieceType::Pawn));

        let expected_moves = vec![(4, 0)];

        let possible_moves = calculate_possible_moves(5, 0, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn white_pawn_cannot_move_two_steps_forward_if_blocked_by_same_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][0] = Some(Piece::White(PieceType::Pawn));
        board[4][0] = Some(Piece::White(PieceType::Pawn)); // Block 2-step move

        let expected_moves = vec![(5, 0)];
        let possible_moves = calculate_possible_moves(6, 0, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert!(possible_moves.len() == 1, "Pawn should not be able to move two steps forward due to block");
        assert_eq!(possible_moves[0].new_pos, expected_moves[0]);
    }

    #[test]
    fn white_pawn_can_move_diagonally_to_capture() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][1] = Some(Piece::White(PieceType::Pawn));
        board[5][2] = Some(Piece::Black(PieceType::Pawn)); // Opponent's pawn to capture diagonally
        board[5][0] = Some(Piece::Black(PieceType::Pawn)); // Opponent's pawn to capture diagonally

        let expected_moves = vec![(5, 2), (5, 0), (5, 1), (4, 1)];

        let possible_moves = calculate_possible_moves(6, 1, &mut board, false, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn white_pawn_can_perform_en_passant() {
        let mut board = vec![vec![None; 8]; 8];
        board[3][0] = Some(Piece::White(PieceType::Pawn));
        board[3][1] = Some(Piece::Black(PieceType::Pawn));

        let expected_moves = vec![(2, 0), (2, 1)];

        let possible_moves = calculate_possible_moves(
            3,
            0,
            &mut board,
            false,
            true,
            &Some(Move {
                current_pos: (1, 1),
                new_pos: (3, 1),
            }),
            &mut CastleState::new(),
        ).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn black_pawn_moves_one_or_two_steps_forward_from_start() {
        let mut board = vec![vec![None; 8]; 8];
        board[1][0] = Some(Piece::Black(PieceType::Pawn));

        let expected_moves = vec![(2, 0), (3, 0)];

        let possible_moves = calculate_possible_moves(1, 0, &mut board, false, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn black_pawn_moves_one_step_not_starting() {
        let mut board = vec![vec![None; 8]; 8];
        board[2][0] = Some(Piece::Black(PieceType::Pawn));

        let expected_moves = vec![(3, 0)];

        let possible_moves = calculate_possible_moves(2, 0, &mut board, false, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn black_pawn_cannot_move_two_steps_forward_if_blocked_by_same_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[1][0] = Some(Piece::Black(PieceType::Pawn));
        board[3][0] = Some(Piece::Black(PieceType::Pawn)); // Block 2-step move

        let expected_moves = vec![(2, 0)];
        let possible_moves = calculate_possible_moves(1, 0, &mut board, false, false, &None, &mut CastleState::new()).unwrap();
        assert!(possible_moves.len() == 1, "Pawn should not be able to move two steps forward due to block");
        assert_eq!(possible_moves[0].new_pos, expected_moves[0]);
    }

    #[test]
    fn black_pawn_can_move_diagonally_to_capture() {
        let mut board = vec![vec![None; 8]; 8];
        board[1][1] = Some(Piece::Black(PieceType::Pawn));
        board[2][2] = Some(Piece::White(PieceType::Pawn)); // Opponent's pawn to capture diagonally
        board[2][0] = Some(Piece::White(PieceType::Pawn)); // Opponent's pawn to capture diagonally

        let expected_moves = vec![(2, 2), (2, 0), (2, 1), (3, 1)];

        let possible_moves = calculate_possible_moves(1, 1, &mut board, false, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn black_pawn_can_perform_en_passant() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][0] = Some(Piece::Black(PieceType::Pawn));
        board[4][1] = Some(Piece::White(PieceType::Pawn));

        let expected_moves = vec![(5, 0), (5, 1)];

        let possible_moves = calculate_possible_moves(
            4,
            0,
            &mut board,
            false,
            false,
            &Some(Move {
                current_pos: (6, 1),
                new_pos: (4, 1),
            }),
            &mut CastleState::new(),
        ).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

}