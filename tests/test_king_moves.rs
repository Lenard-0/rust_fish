
#[cfg(test)]
mod tests {
    use rust_fish::{chess_functionality::moves::calculate_possible_moves, Piece, PieceType};

    #[test]
    fn king_moves_freely_in_all_directions() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::King));

        let expected_moves = vec![
            (3, 3), (3, 4), (3, 5), // Up-left, Up, Up-right
            (4, 3),         (4, 5), // Left, Right
            (5, 3), (5, 4), (5, 5), // Down-left, Down, Down-right
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true, &None).unwrap();
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

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true, &None).unwrap();
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

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, false, &None).unwrap();
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

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true, &None).unwrap();
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

        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, false, &None).unwrap();
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

        let possible_moves = calculate_possible_moves(0, 0, &mut board, true, true, &None).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

}