
#[cfg(test)]
mod tests {
    use rust_fish::{chess_functionality::moves::calculate_possible_moves, Piece, PieceType};

    #[test]
    fn queen_moves_freely() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Queen));

        let expected_moves = vec![
            // Rook-like moves (horizontal & vertical)
            (4, 0), (4, 1), (4, 2), (4, 3), (4, 5), (4, 6), (4, 7),
            (0, 4), (1, 4), (2, 4), (3, 4), (5, 4), (6, 4), (7, 4),
            // Bishop-like moves (diagonal)
            (5, 5), (6, 6), (7, 7), (3, 3), (2, 2), (1, 1), (0, 0),
            (3, 5), (2, 6), (1, 7), (5, 3), (6, 2), (7, 1)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn queen_moves_blocked_by_same_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Queen));
        board[4][6] = Some(Piece::White(PieceType::Pawn)); // Blocks right movement
        board[6][4] = Some(Piece::White(PieceType::Pawn)); // Blocks downward movement
        board[6][6] = Some(Piece::White(PieceType::Pawn)); // Blocks diagonal movement

        let expected_moves = vec![
            // Rook-like moves (horizontal & vertical, stopping before blockers)
            (4, 5), (4, 3), (4, 2), (4, 1), (4, 0),
            (3, 4), (2, 4), (1, 4), (0, 4), (5, 4),
            // Bishop-like moves (diagonal, stopping before blockers)
            (5, 5), (3, 3), (2, 2), (1, 1), (0, 0), (3, 5), (2, 6), (1, 7), (5, 3), (6, 2), (7, 1)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn queen_moves_blocked_by_same_color_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::Queen));
        board[4][6] = Some(Piece::Black(PieceType::Pawn)); // Blocks right movement
        board[6][4] = Some(Piece::Black(PieceType::Pawn)); // Blocks downward movement
        board[6][6] = Some(Piece::Black(PieceType::Pawn)); // Blocks diagonal movement

        let expected_moves = vec![
            // Rook-like moves (horizontal & vertical, stopping before blockers)
            (4, 5), (4, 3), (4, 2), (4, 1), (4, 0),
            (3, 4), (2, 4), (1, 4), (0, 4), (5, 4),
            // Bishop-like moves (diagonal, stopping before blockers)
            (5, 5), (3, 3), (2, 2), (1, 1), (0, 0), (3, 5), (2, 6), (1, 7), (5, 3), (6, 2), (7, 1)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, false, &None).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn queen_can_capture_opponent_piece() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Queen));
        board[6][6] = Some(Piece::Black(PieceType::Pawn)); // Capturable piece

        let expected_moves = vec![
            // Rook-like moves
            (4, 0), (4, 1), (4, 2), (4, 3), (4, 5), (4, 6), (4, 7),
            (0, 4), (1, 4), (2, 4), (3, 4), (5, 4), (6, 4), (7, 4),
            // Bishop-like moves (including capture at 6,6)
            (5, 5), (6, 6), (3, 3), (2, 2), (1, 1), (0, 0),
            (3, 5), (2, 6), (1, 7), (5, 3), (6, 2), (7, 1)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, true, &None).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn queen_can_capture_opponent_piece_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::Queen));
        board[6][6] = Some(Piece::White(PieceType::Pawn)); // Capturable piece

        let expected_moves = vec![
            // Rook-like moves
            (4, 0), (4, 1), (4, 2), (4, 3), (4, 5), (4, 6), (4, 7),
            (0, 4), (1, 4), (2, 4), (3, 4), (5, 4), (6, 4), (7, 4),
            // Bishop-like moves (including capture at 6,6)
            (5, 5), (6, 6), (3, 3), (2, 2), (1, 1), (0, 0),
            (3, 5), (2, 6), (1, 7), (5, 3), (6, 2), (7, 1)
        ];

        let possible_moves = calculate_possible_moves(4, 4, &mut board, false, false, &None).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }

    #[test]
    fn queen_near_board_edge() {
        let mut board = vec![vec![None; 8]; 8];
        board[0][0] = Some(Piece::White(PieceType::Queen));

        let expected_moves = vec![
            // Rook-like moves
            (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
            (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
            // Bishop-like moves
            (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)
        ];

        let possible_moves = calculate_possible_moves(0, 0, &mut board, false, true, &None).unwrap();
        assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
        for m in possible_moves {
            assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
        }
    }
}