
#[cfg(test)]
mod tests {

    use rust_fish_chess_engine::{chess_functionality::moves::{king::CastleState, move_piece::{move_piece, unmove_piece}, Move, SpecialRule}, Piece, PieceType};
    #[test]
    fn can_move_piece_correctly() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][4] = Some(Piece::Black(PieceType::Queen));
        board[7][4] = Some(Piece::White(PieceType::Rook));

        let new_move = Move { current_pos: (6, 4), new_pos: (7, 4), special_rule: None };
        let mut castle_state = CastleState::new();
        let taken_piece = move_piece(&new_move, &mut board, &mut castle_state);
        if let Some(Piece::Black(PieceType::Queen)) = board[7][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Queen)) = board[7][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::White(PieceType::Rook)) = taken_piece {
            assert!(true);
        } else {
            assert!(false);
        }

        unmove_piece(&new_move, &mut board, taken_piece).unwrap();

        if let Some(Piece::White(PieceType::Rook)) = board[7][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Queen)) = board[6][4] {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn can_move_enpassant_correctly() {
        let mut board = vec![vec![None; 8]; 8];
        board[3][4] = Some(Piece::White(PieceType::Pawn));
        board[3][5] = Some(Piece::Black(PieceType::Pawn));

        let new_move = Move { current_pos: (3, 4), new_pos: (2, 5), special_rule: Some(SpecialRule::Enpassant) };
        let mut castle_state = CastleState::new();
        let taken_piece = move_piece(&new_move, &mut board, &mut castle_state);
        if let Some(Piece::White(PieceType::Pawn)) = board[2][5] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Pawn)) = taken_piece {
            assert!(true);
        } else {
            assert!(false);
        }

        unmove_piece(&new_move, &mut board, taken_piece).unwrap();

        if let Some(Piece::White(PieceType::Pawn)) = board[3][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Pawn)) = board[3][5] {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn can_move_castling_correctly() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));
        board[7][0] = Some(Piece::White(PieceType::Rook));
        board[0][4] = Some(Piece::Black(PieceType::King));
        board[0][7] = Some(Piece::Black(PieceType::Rook));
        board[0][0] = Some(Piece::Black(PieceType::Rook));

        // white king side castle
        let new_move = Move { current_pos: (7, 4), new_pos: (7, 6), special_rule: Some(SpecialRule::Castle) };
        let mut castle_state = CastleState::new();
        let taken_piece = move_piece(&new_move, &mut board, &mut castle_state);
        if let Some(Piece::White(PieceType::King)) = board[7][6] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::White(PieceType::Rook)) = board[7][5] {
            assert!(true);
        } else {
            assert!(false);
        }

        unmove_piece(&new_move, &mut board, taken_piece).unwrap();

        if let Some(Piece::White(PieceType::King)) = board[7][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::White(PieceType::Rook)) = board[7][7] {
            assert!(true);
        } else {
            assert!(false);
        }

        // white queen side castle
        let new_move = Move { current_pos: (7, 4), new_pos: (7, 2), special_rule: Some(SpecialRule::Castle) };
        let taken_piece = move_piece(&new_move, &mut board, &mut castle_state);
        if let Some(Piece::White(PieceType::King)) = board[7][2] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::White(PieceType::Rook)) = board[7][3] {
            assert!(true);
        } else {
            assert!(false);
        }

        unmove_piece(&new_move, &mut board, taken_piece).unwrap();

        if let Some(Piece::White(PieceType::King)) = board[7][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::White(PieceType::Rook)) = board[7][0] {
            assert!(true);
        } else {
            assert!(false);
        }

        // black king side castle
        let new_move = Move { current_pos: (0, 4), new_pos: (0, 6), special_rule: Some(SpecialRule::Castle) };
        let taken_piece = move_piece(&new_move, &mut board, &mut castle_state);
        if let Some(Piece::Black(PieceType::King)) = board[0][6] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Rook)) = board[0][5] {
            assert!(true);
        } else {
            assert!(false);
        }

        unmove_piece(&new_move, &mut board, taken_piece).unwrap();

        if let Some(Piece::Black(PieceType::King)) = board[0][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Rook)) = board[0][7] {
            assert!(true);
        } else {
            assert!(false);
        }

        // black queen side castle
        let new_move = Move { current_pos: (0, 4), new_pos: (0, 2), special_rule: Some(SpecialRule::Castle) };
        let taken_piece = move_piece(&new_move, &mut board, &mut castle_state);

        if let Some(Piece::Black(PieceType::King)) = board[0][2] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Rook)) = board[0][3] {
            assert!(true);
        } else {
            assert!(false);
        }

        unmove_piece(&new_move, &mut board, taken_piece).unwrap();

        if let Some(Piece::Black(PieceType::King)) = board[0][4] {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(Piece::Black(PieceType::Rook)) = board[0][0] {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn can_moving_rook_and_king_updates_castle_state() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));
        board[7][0] = Some(Piece::White(PieceType::Rook));
        board[0][4] = Some(Piece::Black(PieceType::King));
        board[0][7] = Some(Piece::Black(PieceType::Rook));
        board[0][0] = Some(Piece::Black(PieceType::Rook));

        // far left white rook moves
        let mut castle_state = CastleState::new();
        let new_move = Move { current_pos: (7, 0), new_pos: (7, 1), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        assert_eq!(castle_state.white_left_rook_moved, true);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.white_left_rook_moved, false);

        // far right white rook moves
        let mut castle_state = CastleState::new();
        let new_move = Move { current_pos: (7, 7), new_pos: (7, 6), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        assert_eq!(castle_state.white_right_rook_moved, true);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.white_right_rook_moved, false);

        // white king moves
        let mut castle_state = CastleState::new();
        let new_move = Move { current_pos: (7, 4), new_pos: (7, 5), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        assert_eq!(castle_state.white_king_moved, true);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.white_king_moved, false);

        // far left black rook moves
        let mut castle_state = CastleState::new();
        let new_move = Move { current_pos: (0, 0), new_pos: (0, 1), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        assert_eq!(castle_state.black_left_rook_moved, true);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.black_left_rook_moved, false);

        // far right black rook moves
        let mut castle_state = CastleState::new();
        let new_move = Move { current_pos: (0, 7), new_pos: (0, 6), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        assert_eq!(castle_state.black_right_rook_moved, true);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.black_right_rook_moved, false);

        // black king moves
        let mut castle_state = CastleState::new();
        let new_move = Move { current_pos: (0, 4), new_pos: (0, 5), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        assert_eq!(castle_state.black_king_moved, true);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.black_king_moved, false);
    }

    #[test]
    fn unmove_does_not_reset_castle_state_if_king_or_rook_had_already_moved_prior() {
        let mut board = vec![vec![None; 8]; 8];
        board[7][4] = Some(Piece::White(PieceType::King));
        board[7][7] = Some(Piece::White(PieceType::Rook));
        board[7][0] = Some(Piece::White(PieceType::Rook));

        let mut castle_state = CastleState::new();
        castle_state.white_king_moved = true;
        castle_state.white_left_rook_moved = true;
        castle_state.white_right_rook_moved = true;

        let new_move = Move { current_pos: (7, 4), new_pos: (7, 5), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.white_king_moved, true);

        let new_move = Move { current_pos: (7, 0), new_pos: (7, 1), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.white_left_rook_moved, true);

        let new_move = Move { current_pos: (7, 7), new_pos: (7, 6), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.white_right_rook_moved, true);


        let mut board = vec![vec![None; 8]; 8];
        board[0][4] = Some(Piece::Black(PieceType::King));
        board[0][7] = Some(Piece::Black(PieceType::Rook));
        board[0][0] = Some(Piece::Black(PieceType::Rook));

        let mut castle_state = CastleState::new();
        castle_state.black_king_moved = true;
        castle_state.black_left_rook_moved = true;
        castle_state.black_right_rook_moved = true;

        let new_move = Move { current_pos: (0, 4), new_pos: (0, 5), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.black_king_moved, true);

        let new_move = Move { current_pos: (0, 0), new_pos: (0, 1), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.black_left_rook_moved, true);

        let new_move = Move { current_pos: (0, 7), new_pos: (0, 6), special_rule: None };
        move_piece(&new_move, &mut board, &mut castle_state);
        unmove_piece(&new_move, &mut board, None).unwrap();
        assert_eq!(castle_state.black_right_rook_moved, true);
    }
}