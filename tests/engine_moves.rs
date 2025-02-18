

#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{king::CastleState, Move}, engine::{get_top_thread, move_thread::MoveThread, rust_fish_choose_move, scoring::determine_move_score}, Piece, PieceType};

    #[test]
    fn engine_can_generate_list_of_next_moves() {
        let board = vec![
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
        let mut castle_state = CastleState::new();
        let start_turn_is_white = false;
        let threads = rust_fish_choose_move(
            0,
            1,
            None,
            vec![],
            board,
            whites_turn,
            &mut castle_state,
            start_turn_is_white
        ).unwrap();

        assert!(threads.len() > 50);
    }

    #[test]
    fn can_determine_correct_score() {
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
        let score = determine_move_score(Some(Piece::Black(PieceType::Rook)), true, &mut board, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(score, 5);
        let score = determine_move_score(Some(Piece::Black(PieceType::Knight)), true, &mut board, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(score, 3);
        let score = determine_move_score(Some(Piece::Black(PieceType::Bishop)), true, &mut board, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(score, 3);
        let score = determine_move_score(Some(Piece::Black(PieceType::Queen)), true, &mut board, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(score, 9);
        let score = determine_move_score(Some(Piece::Black(PieceType::King)), true, &mut board, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(score, 1000);
        let score = determine_move_score(Some(Piece::Black(PieceType::Pawn)), true, &mut board, true, &None, &mut CastleState::new()).unwrap();
        assert_eq!(score, 1);
    }

    #[test]
    fn engine_can_choose_clear_best_move_taking_piece() {
        let board = vec![
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
        let mut castle_state = CastleState::new();
        let start_turn_is_white = false;
        let threads = rust_fish_choose_move(
            0,
            1,
            None,
            vec![],
            board,
            whites_turn,
            &mut castle_state,
            start_turn_is_white
        ).unwrap();

        let top_thread = get_top_thread(threads).unwrap();
        println!("{:?}", top_thread);

        let expected_top_thread = MoveThread {
            moves: vec![Move {
                current_pos: (0, 2),
                new_pos: (2, 4),
                special_rule: None
            }],
            current_score: 5,
            current_board_encoded: "ignore this".to_string()
        };

        assert_eq!(top_thread.moves.len(), 1);
        assert_eq!(top_thread.current_score, 5);
        assert_eq!(top_thread.moves[0], expected_top_thread.moves[0]);
    }
}