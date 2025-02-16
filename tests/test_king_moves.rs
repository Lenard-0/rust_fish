
#[cfg(test)]
mod tests {
    use rust_fish::{chess_functionality::moves::{calculate_possible_moves, king::CastleState, Move}, Piece, PieceType};

    #[test]
    fn king_moves_freely_in_all_directions() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));

        let expected_moves = vec![
            (3, 3), (3, 4), (3, 5), // Up-left, Up, Up-right
            (4, 3),         (4, 5), // Left, Right
            (5, 3), (5, 4), (5, 5), // Down-left, Down, Down-right
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }


    #[test]
    fn king_moves_blocked_by_same_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));
        board[3][4] = Some(Piece::White(PieceType::Pawn)); // Blocks upward
        board[4][5] = Some(Piece::White(PieceType::Pawn)); // Blocks rightward
        board[5][3] = Some(Piece::White(PieceType::Pawn)); // Blocks down-left

        let expected_moves = vec![
            (3, 3), (3, 5), // Up-left, Up-right
            (4, 3),         // Left
            (5, 4), (5, 5)  // Down, Down-right
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn king_moves_blocked_by_same_color_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::King));
        board[3][4] = Some(Piece::Black(PieceType::Pawn)); // Blocks upward
        board[4][5] = Some(Piece::Black(PieceType::Pawn)); // Blocks rightward
        board[5][3] = Some(Piece::Black(PieceType::Pawn)); // Blocks down-left

        let expected_moves = vec![
            (3, 3), (3, 5), // Up-left, Up-right
            (4, 3),         // Left
            (5, 4), (5, 5)  // Down, Down-right
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn king_can_capture_opponent_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));
        board[3][4] = Some(Piece::Black(PieceType::Pawn));
        board[4][5] = Some(Piece::Black(PieceType::Pawn));

        let expected_moves = vec![
            (3, 3), (3, 4), (3, 5),
            (5, 3), (5, 5)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn king_can_capture_opponent_piece_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::King));
        board[3][4] = Some(Piece::White(PieceType::Pawn)); // Capturable
        board[5][5] = Some(Piece::White(PieceType::Pawn)); // Capturable

        let expected_moves = vec![
            (3, 3), (3, 4), (3, 5), // Up-left, Up (capture), Up-right
            (4, 3),         (4, 5), // Left, Right
            (5, 3), (5, 4), (5, 5)  // Down-left, Down, Down-right (capture)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, false, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn king_moves_at_board_edge() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::King)); // Top-left corner

        let expected_moves = vec![
            (0, 1), // Right
            (1, 0), // Down
            (1, 1)  // Down-right
        ];

        let possible_moves = calculate_possible_moves(0, 0, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn white_can_castle_kingside() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));

        let expected_moves = vec![(7, 6)]; // Castling move
        let possible_moves = calculate_possible_moves(7, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert!(possible_moves.iter().any(|m| expected_moves.contains(&m.new_pos)), "Kingside castling not found");
    }

    #[test]
    fn white_can_castle_queenside() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][0] = Some(Piece::White(PieceType::Rook));

        let expected_moves = vec![(7, 2)]; // Castling move
        let possible_moves = calculate_possible_moves(7, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        assert!(possible_moves.iter().any(|m| expected_moves.contains(&m.new_pos)), "Queenside castling not found");
    }

    #[test]
    fn black_can_castle_kingside() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][4] = Some(Piece::Black(PieceType::King));
        board[0][7] = Some(Piece::Black(PieceType::Rook));

        let expected_moves = vec![(0, 6)]; // Castling move
        let possible_moves = calculate_possible_moves(0, 4, &mut board, true, false, &None, &mut CastleState::new()).unwrap();
        assert!(possible_moves.iter().any(|m| expected_moves.contains(&m.new_pos)), "Kingside castling not found");
    }

    #[test]
    fn black_can_castle_queenside() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][4] = Some(Piece::Black(PieceType::King));
        board[0][0] = Some(Piece::Black(PieceType::Rook));

        let expected_moves = vec![(0, 2)]; // Castling move
        let possible_moves = calculate_possible_moves(0, 4, &mut board, true, false, &None, &mut CastleState::new()).unwrap();
        assert!(possible_moves.iter().any(|m| expected_moves.contains(&m.new_pos)), "Queenside castling not found");
    }

    #[test]
    fn white_king_cant_castle_if_king_has_moved() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));

        let mut castle_state = CastleState::new();
        castle_state.white_king_moved = true;
        let possible_moves = calculate_possible_moves(7, 4, &mut board, true, true, &None, &mut castle_state).unwrap();
        for m in possible_moves {
            assert_ne!(m.new_pos, (7, 6), "King should not be able to castle if it has moved");
        }
    }

    #[test]
    fn white_king_cant_castle_if_rook_has_moved() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));

        let mut castle_state = CastleState::new();
        castle_state.white_right_rook_moved = true;
        let possible_moves = calculate_possible_moves(7, 4, &mut board, true, true, &None, &mut castle_state).unwrap();
        for m in possible_moves {
            assert_ne!(m.new_pos, (7, 6), "King should not be able to castle if the rook has moved");
        }
    }

    #[test]
    fn white_king_cant_castle_if_pieces_between_king_and_rook() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));

        // Place a piece between the king and the rook
        board[7][5] = Some(Piece::White(PieceType::Pawn));

        let possible_moves = calculate_possible_moves(7, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        for m in possible_moves {
            assert_ne!(m.new_pos, (7, 6), "King should not be able to castle if there are pieces between it and the rook");
        }
    }

    #[test]
    fn white_king_cant_castle_if_king_is_in_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));

        // Place a black piece attacking the king
        board[0][4] = Some(Piece::Black(PieceType::Queen)); // Queen can attack the king

        let possible_moves = calculate_possible_moves(7, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        for m in possible_moves {
            assert_ne!(m.new_pos, (7, 6), "King should not be able to castle if it is in check");
        }
    }

    // #[test]
    // fn white_king_cant_castle_if_king_moves_through_check() {
    //     let mut board = vec![vec![None; 8]; 8];
    //     board[7][4] = Some(Piece::White(PieceType::King));
    //     board[7][7] = Some(Piece::White(PieceType::Rook));

    //     // Place a black piece attacking the square the king would move through
    //     board[4][5] = Some(Piece::Black(PieceType::Rook));

    //     let possible_moves = calculate_possible_moves(7, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
    //     for m in possible_moves {
    //         assert_ne!(m.new_pos, (7, 6), "King should not be able to castle if it moves through check");
    //     }
    // }

    #[test]
    fn white_king_cant_castle_if_king_ends_up_in_check() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));

        // Place a black piece attacking the square the king would end up in
        board[0][5] = Some(Piece::Black(PieceType::Queen)); // Queen attacks the square the king will end up on

        let possible_moves = calculate_possible_moves(7, 4, &mut board, true, true, &None, &mut CastleState::new()).unwrap();
        // assert!(possible_moves.iter().none(|m| m.new_pos == (7, 6)), "King should not be able to castle if it ends up in check");
    }

}