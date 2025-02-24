
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, check::king_is_checked, king::CastleState, move_piece::move_piece, Move, SpecialRule}, Piece, PieceType};

    #[test]
    fn cannot_move_into_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[6][4] = Some(Piece::Black(PieceType::Queen));

        let expected_moves = vec![(7, 6)];
        let possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn cannot_move_to_cause_check_for_piece_blocking_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[7][6] = Some(Piece::White(PieceType::Rook));
        board[7][4] = Some(Piece::Black(PieceType::Queen));

        let rook_expected_moves = vec![(7, 5), (7, 4)];
        let rook_possible_moves = calculate_possible_moves(7, 6, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(rook_possible_moves.len(), rook_expected_moves.len(), "Expected and actual moves differ in count");
        for m in rook_possible_moves {
            assert!(rook_expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }

        let king_expected_moves = vec![(6, 6), (6, 7)];
        let king_possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(king_possible_moves.len(), king_expected_moves.len(), "Expected and actual moves differ in count");
        for m in king_possible_moves {
            assert!(king_expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn can_avoid_check_by_moving_king() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[7][4] = Some(Piece::Black(PieceType::Queen));

        let expected_moves = vec![(6, 6), (6, 7)];
        let possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn can_avoid_check_by_blocking_with_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[6][6] = Some(Piece::White(PieceType::Rook));
        board[7][4] = Some(Piece::Black(PieceType::Queen));

        let expected_moves = vec![(7, 6)];
        let possible_moves = calculate_possible_moves(6, 6, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn cannot_move_piece_not_resulting_in_block_when_king_is_in_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[0][0] = Some(Piece::White(PieceType::Rook));
        board[7][4] = Some(Piece::Black(PieceType::Queen));

        let possible_moves = calculate_possible_moves(0, 0, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(possible_moves.len(), 0, "Expected and actual moves differ in count");
    }

    #[test]
    fn cannot_avoid_check_by_blocking_with_piece_due_to_check_pin_by_another_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[6][6] = Some(Piece::White(PieceType::Rook));
        board[7][4] = Some(Piece::Black(PieceType::Queen));
        board[0][0] = Some(Piece::Black(PieceType::Bishop));

        let rook_possible_moves = calculate_possible_moves(6, 6, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(rook_possible_moves.len(), 0, "Expected and actual moves differ in count");

        let king_expected_moves = vec![(6, 7)];
        let king_possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(king_possible_moves.len(), king_expected_moves.len(), "Expected and actual moves differ in count");
        for m in king_possible_moves {
            assert!(king_expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn king_cannot_take_piece_resulting_in_check_even_though_piece_is_pinned_with_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[6][6] = Some(Piece::White(PieceType::Rook));
        board[7][4] = Some(Piece::Black(PieceType::Queen));
        board[0][0] = Some(Piece::Black(PieceType::Bishop));

        let rook_possible_moves = calculate_possible_moves(6, 6, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(rook_possible_moves.len(), 0, "Expected and actual moves differ in count");

        let king_expected_moves = vec![(6, 7)];
        let king_possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new(), false).unwrap();
        assert_eq!(king_possible_moves.len(), king_expected_moves.len(), "Expected and actual moves differ in count");
        for m in king_possible_moves {
            assert!(king_expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn special_move_enpassant_removes_defender_resulting_in_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::King));
        board[1][3] = Some(Piece::White(PieceType::Bishop));
        board[3][4] = Some(Piece::White(PieceType::Pawn));
        board[3][5] = Some(Piece::Black(PieceType::Pawn));
        board[5][7] = Some(Piece::Black(PieceType::King));

        let new_move = Move { current_pos: (3, 4), new_pos: (2, 5), special_rule: Some(SpecialRule::Enpassant) };
        move_piece(&new_move, &mut board, &mut CastleState::new());
        assert!(king_is_checked(&mut board, false, &None, &mut CastleState::new()).unwrap());
    }

    #[test]
    fn can_only_move_knight_to_block_check() {
        let mut board = vec![
            vec![Some(Piece::Black(PieceType::Rook)), None, Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Queen)), Some(Piece::Black(PieceType::King)), Some(Piece::Black(PieceType::Bishop)), None, Some(Piece::Black(PieceType::Rook))],
            vec![None, Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), None, None, None, Some(Piece::Black(PieceType::Pawn)), None],
            vec![Some(Piece::Black(PieceType::Pawn)), None, Some(Piece::Black(PieceType::Knight)), None, Some(Piece::White(PieceType::Rook)), Some(Piece::Black(PieceType::Knight)), None, Some(Piece::Black(PieceType::Pawn))],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, Some(Piece::White(PieceType::Bishop)), None, None, None, None, None],
            vec![None, None, None, None, None, Some(Piece::White(PieceType::Knight)), None, None],
            vec![Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), None, Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn))],
            vec![Some(Piece::White(PieceType::Rook)), Some(Piece::White(PieceType::Knight)), Some(Piece::White(PieceType::Bishop)), Some(Piece::White(PieceType::Queen)), None, None, Some(Piece::White(PieceType::King)), None]
        ];

        let whites_turn = false;
        let possible_moves = calculate_possible_moves(
            2,
            2,
            &mut board,
            true,
            whites_turn,
            &None,
            &mut CastleState::new(),
            false
        ).unwrap();

        assert_eq!(possible_moves, vec![Move { current_pos: (2, 2), new_pos: (1, 4), special_rule: None }]);
    }
}