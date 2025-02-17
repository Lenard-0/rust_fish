
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState}, Piece, PieceType};

    #[test]
    fn cannot_move_into_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[6][4] = Some(Piece::Black(PieceType::Queen));

        let expected_moves = vec![(7, 6)];
        let possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
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
        let rook_possible_moves = calculate_possible_moves(7, 6, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(rook_possible_moves.len(), rook_expected_moves.len(), "Expected and actual moves differ in count");
        for m in rook_possible_moves {
            assert!(rook_expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }

        let king_expected_moves = vec![(6, 6), (6, 7)];
        let king_possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
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
        let possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
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
        let possible_moves = calculate_possible_moves(6, 6, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
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

        let possible_moves = calculate_possible_moves(0, 0, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), 0, "Expected and actual moves differ in count");
    }

    #[test]
    fn cannot_avoid_check_by_blocking_with_piece_due_to_check_pin_by_another_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][7] = Some(Piece::White(PieceType::King));
        board[6][6] = Some(Piece::White(PieceType::Rook));
        board[7][4] = Some(Piece::Black(PieceType::Queen));
        board[0][0] = Some(Piece::Black(PieceType::Bishop));

        let rook_possible_moves = calculate_possible_moves(6, 6, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(rook_possible_moves.len(), 0, "Expected and actual moves differ in count");

        let king_expected_moves = vec![(6, 7)];
        let king_possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
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

        let rook_possible_moves = calculate_possible_moves(6, 6, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(rook_possible_moves.len(), 0, "Expected and actual moves differ in count");

        let king_expected_moves = vec![(6, 7)];
        let king_possible_moves = calculate_possible_moves(7, 7, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(king_possible_moves.len(), king_expected_moves.len(), "Expected and actual moves differ in count");
        for m in king_possible_moves {
            assert!(king_expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    // #[test]
    // fn special_move_enpassant_removes_defender_resulting_in_check() {
    //     let mut board = vec![vec![None; 8]; 8];
    //     board[0][0] = Some(Piece::White(PieceType::King));
    //     board[1][3] = Some(Piece::White(PieceType::Bishop));
    //     board[3][4] = Some(Piece::White(PieceType::Pawn));
    //     board[3][5] = Some(Piece::Black(PieceType::Pawn));
    //     board[5][7] = Some(Piece::Black(PieceType::King));


    // }

    // #[test]
    // fn castle_results_in_check() {
    //     let mut board = vec![vec![None; 8]; 8];

    // }
}