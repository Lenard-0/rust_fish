
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{calculate_possible_moves, king::CastleState}, Piece, PieceType};

    #[test]
    fn can_move_piece_correctly() {
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
}