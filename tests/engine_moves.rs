

#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::Move, engine::rust_fish_choose_move, Piece, PieceType};

    #[test]
    fn engine_can_choose_move() {
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
        // let threads = rust_fish_choose_move(board, whites_turn).unwrap();
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
        // let threads = rust_fish_choose_move(board, whites_turn).unwrap();
    }
}