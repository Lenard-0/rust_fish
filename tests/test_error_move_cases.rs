
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState}, Piece, PieceType};

    #[test]
    fn cannot_move_piece_when_wrong_turn() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::White(PieceType::Bishop)); // Bishop in the middle
        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, false, &None, &mut CastleState::new());
        assert!(possible_moves.is_err());
    }

    #[test]
    fn cannot_move_piece_when_wrong_turn_alt_color() {
        let mut board = vec![vec![None; 8]; 8];
        board[4][4] = Some(Piece::Black(PieceType::Bishop)); // Bishop in the middle
        let possible_moves = calculate_possible_moves(4, 4, &mut board, true, true, &None, &mut CastleState::new());
        assert!(possible_moves.is_err());
    }
}