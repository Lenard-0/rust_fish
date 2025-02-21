
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{Move, SpecialRule}, engine::ordering::{get_pawn_attack_map, order_moves}, Piece, PieceType};

    fn empty_board() -> Vec<Vec<Option<Piece>>> {
        vec![vec![None; 8]; 8]
    }

    #[test]
    fn test_order_moves_prioritizes_king_capture() {
        let mut board = empty_board();
        board[0][0] = Some(Piece::Black(PieceType::King));
        board[1][1] = Some(Piece::White(PieceType::Queen));

        let moves = vec![
            Move { current_pos: (1, 1), new_pos: (1, 0), special_rule: None }, // incorrect move
            Move { current_pos: (1, 1), new_pos: (1, 7), special_rule: None },  // incorrect move
            Move { current_pos: (1, 1), new_pos: (0, 0), special_rule: None }, // correct move
        ];
        let ordered_moves = order_moves(moves, &board, true).unwrap();

        assert_eq!(ordered_moves[0].new_pos, (0, 0)); // Capturing king should be first
    }

    #[test]
    fn test_order_moves_prioritizes_promotion() {
        let mut board = empty_board();
        board[6][0] = Some(Piece::White(PieceType::Pawn));
        board[1][0] = Some(Piece::White(PieceType::Queen));

        let moves = vec![
            Move { current_pos: (1, 0), new_pos: (0, 0), special_rule: None }, // incorrect move
            Move { current_pos: (6, 0), new_pos: (7, 0), special_rule: Some(SpecialRule::Promotion(PieceType::Rook)) }, // incorrect move
            Move { current_pos: (6, 0), new_pos: (7, 0), special_rule: Some(SpecialRule::Promotion(PieceType::Queen)) }, // correct move
            Move { current_pos: (1, 0), new_pos: (1, 7), special_rule: None }, // incorrect move
        ];
        let ordered_moves = order_moves(moves.clone(), &board, true).unwrap();

        assert_eq!(ordered_moves[0], moves[2]); // Promotion to queen should be first
        assert_eq!(ordered_moves[1], moves[1]); // Promotion to rook should be second
    }

    #[test]
    fn test_order_moves_prioritizes_castling() {
        let mut board = empty_board();
        board[0][4] = Some(Piece::White(PieceType::King));
        board[0][7] = Some(Piece::White(PieceType::Rook));

        let moves = vec![
            Move { current_pos: (0, 4), new_pos: (0, 5), special_rule: None }, // incorrect move
            Move { current_pos: (0, 4), new_pos: (0, 6), special_rule: Some(SpecialRule::Castle) } // correct move
        ];
        let ordered_moves = order_moves(moves.clone(), &board, true).unwrap();

        assert_eq!(ordered_moves[0], moves[1]); // Castling should be first
    }

    #[test]
    fn test_pawn_attack_map_white() {
        let mut board = vec![vec![None; 8]; 8];
        board[6][3] = Some(Piece::White(PieceType::Pawn));
        board[6][5] = Some(Piece::White(PieceType::Pawn));
        let attack_map = get_pawn_attack_map(&board, true);

        assert!(attack_map[5][2]); // Left diagonal attack
        assert!(attack_map[5][4]); // Right diagonal attack
        assert!(attack_map[5][4]); // Left diagonal attack
        assert!(attack_map[5][6]); // Right diagonal attack
        assert!(!attack_map[5][3]); // Forward movement isn't an attack
    }

    #[test]
    fn test_pawn_attack_map_black() {
        let mut board = vec![vec![None; 8]; 8];
        board[1][3] = Some(Piece::Black(PieceType::Pawn));
        board[1][5] = Some(Piece::Black(PieceType::Pawn));
        let attack_map = get_pawn_attack_map(&board, false);

        assert!(attack_map[2][2]); // Left diagonal attack
        assert!(attack_map[2][4]); // Right diagonal attack
        assert!(attack_map[2][4]); // Left diagonal attack
        assert!(attack_map[2][6]); // Right diagonal attack
        assert!(!attack_map[2][3]); // Forward movement isn't an attack
    }

    #[test]
    fn test_order_moves_penalizes_moving_into_pawn_attack() {
        let mut board = empty_board();
        board[6][5] = Some(Piece::White(PieceType::Rook));
        board[5][5] = Some(Piece::Black(PieceType::Pawn));

        let moves = vec![
            Move { current_pos: (6, 5), new_pos: (6, 4), special_rule: None }, // worst move
            Move { current_pos: (6, 5), new_pos: (6, 0), special_rule: None }, // middle move
            Move { current_pos: (6, 5), new_pos: (5, 5), special_rule: None }, // correct move
        ];
        let ordered_moves = order_moves(moves.clone(), &board, true).unwrap();

        assert_eq!(ordered_moves[0], moves[2]); // Taking pawn should be first
        assert_eq!(ordered_moves[2], moves[0]); // Moving into pawn attack should be last
    }

    #[test]
    fn test_order_moves_prioritizes_capturing_high_value_piece_with_low_value() {
        let mut board = empty_board();
        board[3][3] = Some(Piece::Black(PieceType::Queen));
        board[4][4] = Some(Piece::White(PieceType::Pawn));
        board[3][2] = Some(Piece::White(PieceType::Rook));

        let moves = vec![
            // rook capture move
            Move { current_pos: (3, 2), new_pos: (3, 3), special_rule: None }, // middle move
            // rook moves aimlessly
            Move { current_pos: (3, 2), new_pos: (3, 4), special_rule: None }, // incorrect move
            Move { current_pos: (4, 4), new_pos: (3, 3), special_rule: None } // correct move
        ];
        let ordered_moves = order_moves(moves.clone(), &board, true).unwrap();

        assert_eq!(ordered_moves[0], moves[2]); // Capturing queen with pawn should be first
        assert_eq!(ordered_moves[1], moves[0]); // Capturing queen with rook should be second
    }
}