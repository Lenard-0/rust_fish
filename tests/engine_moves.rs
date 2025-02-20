

#[cfg(test)]
mod tests {
    use std::f32::INFINITY;

    use rust_fish_chess_engine::{chess_functionality::moves::{king::CastleState, Move}, engine::{scoring::evaluate_board, search_for_moves}, Piece, PieceType};

    #[test]
    fn engine_can_generate_next_best_move() {
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

        let mut castle_state = CastleState::new();
        let whites_turn = false;
        let alpha = -INFINITY as i32;
        let beta = INFINITY as i32;
        let _engine_calculation = search_for_moves(
            3,
            alpha,
            beta,
            &mut board,
            whites_turn,
            &None,
            &mut castle_state
        ).unwrap();
    }

    #[test]
    fn can_determine_correct_score() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::Rook));
        let score = evaluate_board(&board, true).unwrap();
        assert_eq!(score, 500);

        board[0][0] = Some(Piece::White(PieceType::Knight));
        let score = evaluate_board(&board, true).unwrap();
        assert_eq!(score, 300);

        board[0][0] = Some(Piece::White(PieceType::Bishop));
        let score = evaluate_board(&board, true).unwrap();
        assert_eq!(score, 300);

        board[0][0] = Some(Piece::White(PieceType::Queen));
        let score = evaluate_board(&board, true).unwrap();
        assert_eq!(score, 900);

        board[0][0] = Some(Piece::White(PieceType::Pawn));
        let score = evaluate_board(&board, true).unwrap();
        assert_eq!(score, 100);

        board[0][0] = Some(Piece::White(PieceType::King));
        let score = evaluate_board(&board, true).unwrap();
        assert_eq!(score, 0);
    }

    #[test]
    fn can_determine_total_correct_score() {
        // start empty board
        let mut board = vec![vec![None; 8]; 8];
        let score = evaluate_board(&board, true).unwrap();
        assert_eq!(score, 0);

        // add a few pieces
        board[0][0] = Some(Piece::White(PieceType::Rook));
        board[0][1] = Some(Piece::White(PieceType::Knight));
        board[0][2] = Some(Piece::White(PieceType::Bishop));
        board[0][3] = Some(Piece::White(PieceType::Queen));

        let score = evaluate_board(&board, true).unwrap();
        assert_eq!(score, 2000);
    }

    #[test]
    fn engine_can_choose_clear_best_move_taking_piece_with_single_depth() {
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
        let current_board_score = evaluate_board(&board, whites_turn).unwrap();
        assert_eq!(current_board_score, -200);
        let mut castle_state = CastleState::new();
        let alpha = -10000 as i32;
        let beta = 10000 as i32;
        let engine_calculation = search_for_moves(
            1,
            alpha,
            beta,
            &mut board,
            whites_turn,
            &None,
            &mut castle_state
        ).unwrap();

        let expected_top_thread = Move {
            current_pos: (0, 2),
            new_pos: (2, 4),
            special_rule: None
        };
        println!("{:?}", engine_calculation);
        assert_eq!(engine_calculation.score, 300); // expecting rook to be taken which is +500 to the starting -200
        assert_eq!(engine_calculation.best_move.unwrap(), expected_top_thread);
    }

    #[test]
    fn engine_can_choose_clear_best_move_taking_piece_with_depth_of_3() {
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
        let current_board_score = evaluate_board(&board, whites_turn).unwrap();
        assert_eq!(current_board_score, -200);
        let mut castle_state = CastleState::new();
        let alpha = -10000 as i32;
        let beta = 10000 as i32;
        let engine_calculation = search_for_moves(
            6,
            alpha,
            beta,
            &mut board,
            whites_turn,
            &None,
            &mut castle_state
        ).unwrap();

        let expected_top_thread = Move {
            current_pos: (0, 2),
            new_pos: (2, 4),
            special_rule: None
        };
        println!("{:?}", engine_calculation);
        // assert_eq!(engine_calculation.score, 100);
        assert_eq!(engine_calculation.best_move.unwrap(), expected_top_thread);
    }

    #[test]
    fn engine_can_choose_clear_best_move_over_multiple_steps_mate_in_3() {
        let mut board = vec![
            vec![None, Some(Piece::Black(PieceType::Pawn)), None,None,None,None,None, Some(Piece::Black(PieceType::King))],
            vec![None, Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn))],
            vec![None, Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn))],
            vec![None, None, None, None, None, Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn))],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![None, Some(Piece::White(PieceType::King)), None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, Some(Piece::White(PieceType::Rook))]
        ];



        let whites_turn = true;
        let mut castle_state = CastleState::new();
        let alpha = -10000 as i32;
        let beta = 10000 as i32;
        let engine_calculation = search_for_moves(
            7, // 1 extra depth to ensure mate in 3 and to ensure it doesn't fail past this
            alpha,
            beta,
            &mut board,
            whites_turn,
            &None,
            &mut castle_state
        ).unwrap();

        let expected_moves = vec![
            Move {
                current_pos: (7, 7),
                new_pos: (7, 0),
                special_rule: None
            },
            Move {
                current_pos: (7, 0),
                new_pos: (0, 0),
                special_rule: None
            },
            Move {
                current_pos: (0, 0),
                new_pos: (0, 1),
                special_rule: None
            },
        ];

        // assert_eq!(engine_calculation.move_sequence.len(), 3);
        // assert_eq!(engine_calculation.score, INFINITY as i32);
        println!("{:?}", engine_calculation);
        assert_eq!(engine_calculation.best_move.unwrap(), expected_moves[0]);
        assert_eq!(engine_calculation.move_sequence[0], expected_moves[0]);
        // assert_eq!(engine_calculation.move_sequence[1], expected_moves[1]);
        // assert_eq!(engine_calculation.move_sequence[2], expected_moves[2]);
    }
}