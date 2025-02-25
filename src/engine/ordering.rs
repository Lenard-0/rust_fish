use crate::{chess_functionality::moves::{Move, SpecialRule}, Piece, PieceType};


pub fn order_moves(
    moves: Vec<Move>,
    board: &Vec<Vec<Option<Piece>>>,
    whites_turn: bool
) -> Result<Vec<Move>, String> {
    let mut ordered_moves = Vec::new();
    for m in moves {
        let move_piece_type = match board[m.current_pos.0][m.current_pos.1].clone() {
            Some(Piece::White(piece_type)) | Some(Piece::Black(piece_type)) => piece_type,
            None => return Err(format!("Expected piece, but none found on current position: {:#?} {:?}", m, board)),
        };

        let capture_piece_type = match &board[m.new_pos.0][m.new_pos.1] {
            Some(Piece::White(piece_type)) | Some(Piece::Black(piece_type)) => Some(piece_type),
            None => None,
        };

        // Prioritise capturing opponent's most valuable pieces with our least valuable pieces
        let mut move_score_guess = match capture_piece_type {
            // would result in mate so should be prioritised
            Some(PieceType::King) => 10000,
            // capturing a high value piece by a low value piece maximised likely to end well
            Some(capture_piece_type) => 10 * capture_piece_type.get_value() - move_piece_type.get_value(),
            None => 0,
        };


        match &m.special_rule {
            // promoting a pawn is likely to be good
            Some(SpecialRule::Promotion(piece_type)) => move_score_guess += piece_type.get_value(),
            // castling is generally good
            Some(SpecialRule::Castle) => move_score_guess += 50,
            _ => {}
        };

        // Penalise moving our pieces to a square attacked by an opponent pawn
        let pawn_attack_map = get_pawn_attack_map(board, !whites_turn);
        if pawn_attack_map[m.new_pos.0][m.new_pos.1] {
            move_score_guess -= 100;
        }

        ordered_moves.push((m, move_score_guess));
    }

    ordered_moves.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(ordered_moves.into_iter().map(|(m, _)| m).collect())
}

pub fn get_pawn_attack_map(board: &Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Vec<Vec<bool>> {
    let mut attack_map = vec![vec![false; 8]; 8];
    let direction = if whites_turn { -1 } else { 1 }; // White pawns move up (-1), Black pawns move down (+1)

    for ir in 0..8 {
        for ic in 0..8 {
            if let Some(Piece::White(PieceType::Pawn)) = board[ir][ic] {
                if whites_turn && ir > 0 {
                    if ic > 0 {
                        attack_map[ir - 1][ic - 1] = true;
                    }
                    if ic < 7 {
                        attack_map[ir - 1][ic + 1] = true;
                    }
                }
            } else if let Some(Piece::Black(PieceType::Pawn)) = board[ir][ic] {
                if !whites_turn && ir < 7 {
                    if ic > 0 {
                        attack_map[ir + 1][ic - 1] = true;
                    }
                    if ic < 7 {
                        attack_map[ir + 1][ic + 1] = true;
                    }
                }
            }
        }
    }
    attack_map
}