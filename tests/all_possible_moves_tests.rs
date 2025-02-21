
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{chess_functionality::moves::{check::all_possible_moves, king::CastleState, Move, SpecialRule}, Piece, PieceType};


    #[test]
    fn calculates_only_possible_moves_avoiding_impossible_castle() {
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
        castle_state.white_king_moved = true;
        castle_state.black_king_moved = true;
        let all_moves = all_possible_moves(
            &mut board,
            whites_turn,
            &None,
            &mut castle_state,
            false
        ).unwrap();

        // assert no moves are special move castle
        for m in all_moves {
            assert_ne!(m.special_rule, Some(SpecialRule::Castle));
        }

        let all_moves = all_possible_moves(
            &mut board,
            !whites_turn,
            &None,
            &mut castle_state,
            false
        ).unwrap();

        // assert no moves are special move castle
        for m in all_moves {
            assert_ne!(m.special_rule, Some(SpecialRule::Castle));
        }
    }
}