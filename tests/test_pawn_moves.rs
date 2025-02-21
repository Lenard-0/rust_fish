
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState, move_piece::unmove_piece, Move, SpecialRule}, Piece, PieceType};
    use rust_fish_chess_engine::chess_functionality::moves::move_piece::move_piece;
    #[test]
    fn white_pawn_moves_one_or_two_steps_forward_from_start() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][0] = Some(Piece::White(PieceType::Pawn));

        let expected_moves = vec![(5, 0), (4, 0)];

        let possible_moves = calculate_possible_moves(6, 0, &mut board, false, true, &None, &mut CastleState::new(), false).unwrap();
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

        let possible_moves = calculate_possible_moves(5, 0, &mut board, false, true, &None, &mut CastleState::new(), false).unwrap();
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
        let possible_moves = calculate_possible_moves(6, 0, &mut board, false, true, &None, &mut CastleState::new(), false).unwrap();
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

        let possible_moves = calculate_possible_moves(6, 1, &mut board, false, true, &None, &mut CastleState::new(), false).unwrap();
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
                special_rule: None
            }),
            &mut CastleState::new(),
            false
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

        let possible_moves = calculate_possible_moves(1, 0, &mut board, false, false, &None, &mut CastleState::new(), false).unwrap();
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

        let possible_moves = calculate_possible_moves(2, 0, &mut board, false, false, &None, &mut CastleState::new(), false).unwrap();
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
        let possible_moves = calculate_possible_moves(1, 0, &mut board, false, false, &None, &mut CastleState::new(), false).unwrap();
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

        let possible_moves = calculate_possible_moves(1, 1, &mut board, false, false, &None, &mut CastleState::new(), false).unwrap();
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
                special_rule: None
            }),
            &mut CastleState::new(),
            false
        ).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn white_pawn_can_promote_moving_one_up() {
        let mut board = vec![vec![None; 8]; 8];
        board[1][0] = Some(Piece::White(PieceType::Pawn));

        let expected_moves = vec![
            Move {
                current_pos: (1, 0),
                new_pos: (0, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (1, 0),
                new_pos: (0, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (1, 0),
                new_pos: (0, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (1, 0),
                new_pos: (0, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            }
        ];

        let possible_moves = calculate_possible_moves(1, 0, &mut board, false, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m), "Unexpected move: {:?}", m);
        }
    }

    #[test]
    fn white_pawn_can_promote_moving_taking_either_side() {
        let mut board = vec![vec![None; 8]; 8];
        board[1][1] = Some(Piece::White(PieceType::Pawn));
        board[0][0] = Some(Piece::Black(PieceType::Pawn));
        board[0][2] = Some(Piece::Black(PieceType::Pawn));

        let expected_moves = vec![
            Move {
                current_pos: (1, 1),
                new_pos: (0, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 2),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 2),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 2),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 2),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 1),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 1),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 1),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (1, 1),
                new_pos: (0, 1),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            }
        ];

        let possible_moves = calculate_possible_moves(1, 1, &mut board, false, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m), "Unexpected move: {:?}", m);
        }
    }

    #[test]
    fn black_pawn_one_up_can_promote() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][7] = Some(Piece::Black(PieceType::Pawn));

        let expected_moves = vec![
            Move {
                current_pos: (6, 7),
                new_pos: (7, 7),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (6, 7),
                new_pos: (7, 7),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (6, 7),
                new_pos: (7, 7),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (6, 7),
                new_pos: (7, 7),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            }
        ];

        let possible_moves = calculate_possible_moves(6, 7, &mut board, false, false, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m), "Unexpected move: {:?}", m);
        }
    }

    #[test]
    fn black_pawn_can_promote() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][1] = Some(Piece::Black(PieceType::Pawn));
        board[7][0] = Some(Piece::White(PieceType::Pawn));
        board[7][2] = Some(Piece::White(PieceType::Pawn));

        let expected_moves = vec![
            Move {
                current_pos: (6, 1),
                new_pos: (7, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 0),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 2),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 2),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 2),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 2),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 1),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 1),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 1),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (6, 1),
                new_pos: (7, 1),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            },
        ];

        let possible_moves = calculate_possible_moves(6, 1, &mut board, false, false, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m), "Unexpected move: {:?}", m);
        }
    }

    #[test]
    fn promotion_move_works_correctly() {
        let mut board = vec![vec![None; 8]; 8];
        board[1][0] = Some(Piece::White(PieceType::Pawn));

        let new_move = Move {
            current_pos: (1, 0),
            new_pos: (0, 0),
            special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
        };
        let mut expected_board = vec![vec![None; 8]; 8];
        expected_board[0][0] = Some(Piece::White(PieceType::Knight));
        move_piece(&new_move, &mut board, &mut CastleState::new());
        assert_eq!(board, expected_board);
        unmove_piece(&new_move, &mut board, None, &mut CastleState::new()).unwrap();

        let new_move = Move {
            current_pos: (1, 0),
            new_pos: (0, 0),
            special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
        };
        expected_board[0][0] = Some(Piece::White(PieceType::Bishop));
        move_piece(&new_move, &mut board, &mut CastleState::new());
        assert_eq!(board, expected_board);
        unmove_piece(&new_move, &mut board, None, &mut CastleState::new()).unwrap();

        let new_move = Move {
            current_pos: (1, 0),
            new_pos: (0, 0),
            special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
        };
        expected_board[0][0] = Some(Piece::White(PieceType::Rook));
        move_piece(&new_move, &mut board, &mut CastleState::new());
        assert_eq!(board, expected_board);
        unmove_piece(&new_move, &mut board, None, &mut CastleState::new()).unwrap();

        let new_move = Move {
            current_pos: (1, 0),
            new_pos: (0, 0),
            special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
        };
        expected_board[0][0] = Some(Piece::White(PieceType::Queen));
        move_piece(&new_move, &mut board, &mut CastleState::new());
        assert_eq!(board, expected_board);
        unmove_piece(&new_move, &mut board, None, &mut CastleState::new()).unwrap();
    }

    #[test]
    fn black_pawn_one_up_can_move_and_promote_correctly() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][7] = Some(Piece::Black(PieceType::Pawn));

        let expected_moves = vec![
            Move {
                current_pos: (6, 7),
                new_pos: (7, 7),
                special_rule: Some(SpecialRule::Promotion(PieceType::Knight))
            },
            Move {
                current_pos: (6, 7),
                new_pos: (7, 7),
                special_rule: Some(SpecialRule::Promotion(PieceType::Bishop))
            },
            Move {
                current_pos: (6, 7),
                new_pos: (7, 7),
                special_rule: Some(SpecialRule::Promotion(PieceType::Rook))
            },
            Move {
                current_pos: (6, 7),
                new_pos: (7, 7),
                special_rule: Some(SpecialRule::Promotion(PieceType::Queen))
            }
        ];

        let possible_moves = calculate_possible_moves(6, 7, &mut board, false, false, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in &possible_moves {
            assert!(expected_moves.contains(m), "Unexpected move: {:?}", m);
        }

        move_piece(&possible_moves[0], &mut board, &mut CastleState::new());
        assert_eq!(board[7][7], Some(Piece::Black(PieceType::Queen)));
        assert_eq!(board[6][7], None);
        unmove_piece(&possible_moves[0], &mut board, None, &mut CastleState::new()).unwrap();
        assert_eq!(board[7][7], None);
        assert_eq!(board[6][7], Some(Piece::Black(PieceType::Pawn)));

        move_piece(&possible_moves[1], &mut board, &mut CastleState::new());
        assert_eq!(board[7][7], Some(Piece::Black(PieceType::Rook)));
        assert_eq!(board[6][7], None);
        unmove_piece(&possible_moves[1], &mut board, None, &mut CastleState::new()).unwrap();
        assert_eq!(board[7][7], None);
        assert_eq!(board[6][7], Some(Piece::Black(PieceType::Pawn)));

        move_piece(&possible_moves[2], &mut board, &mut CastleState::new());
        assert_eq!(board[7][7], Some(Piece::Black(PieceType::Bishop)));
        assert_eq!(board[6][7], None);
        unmove_piece(&possible_moves[2], &mut board, None, &mut CastleState::new()).unwrap();
        assert_eq!(board[7][7], None);
        assert_eq!(board[6][7], Some(Piece::Black(PieceType::Pawn)));

        move_piece(&possible_moves[3], &mut board, &mut CastleState::new());
        assert_eq!(board[7][7], Some(Piece::Black(PieceType::Knight)));
        assert_eq!(board[6][7], None);
        unmove_piece(&possible_moves[3], &mut board, None, &mut CastleState::new()).unwrap();
        assert_eq!(board[7][7], None);
        assert_eq!(board[6][7], Some(Piece::Black(PieceType::Pawn)));
    }
}