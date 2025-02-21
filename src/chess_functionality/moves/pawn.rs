use crate::{Piece, PieceType};
use super::{Move, SpecialRule};

pub fn calculate_pawn_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
    previous_move: &Option<Move>
) -> Result<Vec<Move>, String> {
    let mut possible_moves = Vec::new();
    if whites_turn {
        let tile_one_up = board[ir - 1][ic].clone();
        if tile_one_up.is_none() {
            match ir - 1 == 0 {
                true => possible_moves.extend(Move::all_possible_promotion_moves((ir, ic), (ir - 1, ic))),
                false => possible_moves.push(Move::from_position((ir, ic), (ir - 1, ic))?)
            };
        }
        if ir == 6 {
            if tile_one_up.is_none() && board[ir - 2][ic].is_none() {
                possible_moves.push(Move::from_position((ir, ic), (ir - 2, ic))?);
            }
        }

        //take left
        if ic != 0 {
            let tile = &board[ir - 1][ic - 1];
            match tile {
                Some(Piece::Black(_)) if ir - 1 == 0 => possible_moves.extend(Move::all_possible_promotion_moves((ir, ic), (ir - 1, ic - 1))),
                Some(Piece::Black(_)) => possible_moves.push(Move::from_position((ir, ic), (ir - 1, ic - 1))?),
                _ => {}
            };
        }
        // take right
        if ic != 7 {
            let tile = &board[ir - 1][ic + 1];
            match tile {
                Some(Piece::Black(_)) if ir - 1 == 0 => possible_moves.extend(Move::all_possible_promotion_moves((ir, ic), (ir - 1, ic + 1))),
                Some(Piece::Black(_)) => possible_moves.push(Move::from_position((ir, ic), (ir - 1, ic + 1))?),
                _ => {}
            };
        }

        // enpassant
        if let Some(Move { current_pos: (prev_starting_ir, ..), new_pos: (prev_current_ir, prev_current_ic), .. }) = previous_move {
            // if old ir is starting rank for black pawn and new ir is 2 up
            // and piece is to the left or right of the pawn and that piece is a black pawn
            if *prev_starting_ir == 1
                && *prev_current_ir == 3
                && *prev_current_ir == ir
                && *prev_current_ic < board[0].len() // Prevent out-of-bounds
                && ((ic > 0 && *prev_current_ic == ic - 1) || (*prev_current_ic == ic + 1)) // Ensure valid left/right move
            {
                if ir > 0 { // Prevent underflow for `ir - 1`
                    match board[*prev_current_ir][*prev_current_ic] {
                        Some(Piece::Black(PieceType::Pawn)) => {
                            possible_moves.push(Move {
                                current_pos: (ir, ic),
                                new_pos: (ir - 1, *prev_current_ic),
                                special_rule: Some(SpecialRule::Enpassant),
                            });
                        },
                        _ => {}
                    };
                }
            }
        }
    } else {
        if ir == 7 {
            println!("board: {:?}", board);
        }
        let tile_one_up = board[ir + 1][ic].clone();
        if tile_one_up.is_none() {
            match ir + 1 == 7 {
                true => possible_moves.extend(Move::all_possible_promotion_moves((ir, ic), (ir + 1, ic))),
                false => possible_moves.push(Move::from_position((ir, ic), (ir + 1, ic))?)
            };
        }
        if ir == 1 {
            let tile_two_up = &board[ir + 2][ic];
            if tile_one_up.is_none() && tile_two_up.is_none() {
                possible_moves.push(Move::from_position((ir, ic), (ir + 2, ic))?);
            }
        }

        //take right
        if ic != 7 {
            let tile = &board[ir + 1][ic + 1];
            match tile {
                Some(Piece::White(_)) if ir + 1 == 7 => possible_moves.extend(Move::all_possible_promotion_moves((ir, ic), (ir + 1, ic + 1))),
                Some(Piece::White(_)) => possible_moves.push(Move::from_position((ir, ic), (ir + 1, ic + 1))?),
                _ => {}
            };
        }
        // take left
        if ic != 0 {
            let tile = &board[ir + 1][ic - 1];
            match tile {
                Some(Piece::White(_)) if ir + 1 == 7 => possible_moves.extend(Move::all_possible_promotion_moves((ir, ic), (ir + 1, ic - 1))),
                Some(Piece::White(_)) => possible_moves.push(Move::from_position((ir, ic), (ir + 1, ic - 1))?),
                _ => {}
            };
        }

        // black enpassant

        if let Some(Move { current_pos: (prev_starting_ir, ..), new_pos: (prev_current_ir, prev_current_ic), .. }) = previous_move {
            // if old ir is starting rank for black pawn and new ir is 2 up
            // and piece is to the left or right of the pawn and that piece is a black pawn
            if *prev_starting_ir == 6
                && *prev_current_ir == 4
                && *prev_current_ir == ir
                && *prev_current_ic < board[0].len() // Prevent out-of-bounds
                && ((ic > 0 && *prev_current_ic == ic - 1) || (*prev_current_ic == ic + 1)) // Ensure valid left/right move
            {
                if ir < board.len() - 1 { // Prevent overflow for `ir + 1`
                    match board[*prev_current_ir][*prev_current_ic] {
                        Some(Piece::White(PieceType::Pawn)) => {
                            possible_moves.push(Move {
                                current_pos: (ir, ic),
                                new_pos: (ir + 1, *prev_current_ic), // Move down for black en passant
                                special_rule: Some(SpecialRule::Enpassant),
                            });
                        },
                        _ => {}
                    };
                }
    }
        }
    }

    return Ok(possible_moves)
}