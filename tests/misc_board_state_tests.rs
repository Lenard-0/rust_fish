
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState}, Piece, PieceType};

    #[test]
    fn castling_infinite_loop_does_not_break_code() {
        let mut board = vec![
            vec![Some(Piece::Black(PieceType::Rook)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Queen)), Some(Piece::Black(PieceType::King)), None, None, Some(Piece::Black(PieceType::Rook))],
            vec![Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), None, Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn))],
            vec![None, None, None, None, None, Some(Piece::Black(PieceType::Knight)), None, None],
            vec![None, None, Some(Piece::Black(PieceType::Bishop)), None, Some(Piece::Black(PieceType::Pawn)), None, None, None],
            vec![None, None, Some(Piece::White(PieceType::Bishop)), None, Some(Piece::White(PieceType::Pawn)), None, None, None],
            vec![None, None, None, None, None, Some(Piece::White(PieceType::Knight)), None, None],
            vec![Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), None, Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn))],
            vec![Some(Piece::White(PieceType::Rook)), Some(Piece::White(PieceType::Knight)), Some(Piece::White(PieceType::Bishop)), Some(Piece::White(PieceType::Queen)), Some(Piece::White(PieceType::King)), None, None, Some(Piece::White(PieceType::Rook))]
        ];

        let white_king_pos = (7, 4);
        let whites_turn = true;
        let mut castle_state = CastleState::new();
        calculate_possible_moves(
            white_king_pos.0,
            white_king_pos.1,
            &mut board,
            false,
            whites_turn,
            &None,
            &mut castle_state,
            false
        ).unwrap();
    }

    #[test]
    fn can_only_move_to_stop_check() {
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

        let dark_black_bishop_pos = (0, 5);
        let whites_turn = false;
        let mut castle_state = CastleState::new();
        let possible_moves = calculate_possible_moves(
            dark_black_bishop_pos.0,
            dark_black_bishop_pos.1,
            &mut board,
            true,
            whites_turn,
            &None,
            &mut castle_state,
            false
        ).unwrap();

        let expected_moves = vec![(1, 4)];

        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        assert_eq!(possible_moves[0].new_pos, expected_moves[0]);
    }
}