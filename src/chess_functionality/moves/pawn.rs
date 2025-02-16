use crate::Piece;

use super::Move;

pub fn calculate_pawn_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Result<Vec<Move>, String> {
    return Ok((vec![]))
    // let mut possible_moves = Vec::new();

    // if whites_turn {
    //     let tile_one_up = board[ir - 1][ic].clone();
    //     if tile_one_up.is_none() {
    //         possible_moves.push(Move {
    //             current_pos: (ir, ic),
    //             new_pos: (ir - 1, ic),
    //             taken_piece: None,
    //             check: move_results_in_check((ir, ic), (ir - 1, ic), board[ir - 1][ic].clone(), board, whites_turn)?,
    //             special_rule: None
    //         })
    //     }
    //     if ir == 6 { // starting rank
    //         if tile_one_up.is_none() && board[ir - 2][ic].is_none() {
    //             possible_moves.push(Move {
    //                 current_pos: (ir, ic),
    //                 new_pos: (ir - 2, ic),
    //                 taken_piece: board[ir - 2][ic].clone(),
    //                 check: move_results_in_check((ir, ic), (ir - 2, ic), board[ir - 1][ic].clone(), board, whites_turn)?,
    //                 special_rule: None
    //             })
    //         }
    //     }

    //     //take left
    //     if ic != 0 {
    //         let tile = &board[ir - 1][ic - 1];
    //         match tile {
    //             Some(Piece::Black(_)) => possible_moves.push(Move {
    //                 current_pos: (ir, ic),
    //                 new_pos: (ir - 1, ic - 1),
    //                 taken_piece: board[ir - 1][ic - 1].clone(),
    //                 check: move_results_in_check((ir, ic), (ir - 1, ic - 1), board[ir - 1][ic - 1].clone(), board, whites_turn)?,
    //                 special_rule: None
    //             }),
    //             _ => {}
    //         }
    //     }
    //     // take right
    //     if ic != 7 {
    //         let tile = &board[ir - 1][ic + 1];
    //         match tile {
    //             Some(Piece::Black(_)) => possible_moves.push(Move {
    //                 current_pos: (ir, ic),
    //                 new_pos: (ir - 1, ic + 1),
    //                 taken_piece: board[ir - 1][ic + 1].clone(),
    //                 check: move_results_in_check((ir, ic), (ir - 1, ic + 1), board[ir - 1][ic + 1].clone(), board, whites_turn)?,
    //                 special_rule: None
    //             }),
    //             _ => {}
    //         }
    //     }
    // } else {
    //     let tile_one_up = board[ir + 1][ic].clone();
    //     if tile_one_up.is_none() {
    //         possible_moves.push(Move {
    //             current_pos: (ir, ic),
    //             new_pos: (ir - 1, ic),
    //             taken_piece: board[ir - 1][ic].clone(),
    //             check: move_results_in_check((ir, ic), (ir - 1, ic), board[ir - 1][ic].clone(), board, whites_turn)?,
    //             special_rule: None
    //         });
    //     }
    //     if ir == 6 { // starting rank
    //         let tile_two_up = &board[ir + 2][ic];
    //         if tile_one_up.is_none() && tile_two_up.is_none() {
    //             possible_moves.push(Move {
    //                 current_pos: (ir, ic),
    //                 new_pos: (ir - 2, ic),
    //                 taken_piece: board[ir - 2][ic].clone(),
    //                 check: move_results_in_check((ir, ic), (ir - 2, ic), board[ir - 2][ic].clone(), board, whites_turn)?,
    //                 special_rule: None
    //             });
    //         }
    //     }

    //     //take left
    //     if ic != 7 {
    //         let tile = &board[ir + 1][ic + 1];
    //         match tile {
    //             Some(Piece::Black(_)) => possible_moves.push(Move {
    //                 current_pos: (ir, ic),
    //                 new_pos: (ir + 1, ic + 1),
    //                 taken_piece: board[ir + 1][ic + 1].clone(),
    //                 check: move_results_in_check((ir, ic), (ir + 1, ic + 1), board[ir + 1][ic + 1].clone(), board, whites_turn)?,
    //                 special_rule: None
    //             }),
    //             _ => {}
    //         }
    //     }
    //     // take right
    //     if ic != 0 {
    //         let tile = &board[ir + 1][ic - 1];
    //         match tile {
    //             Some(Piece::Black(_)) => possible_moves.push(Move {
    //                 current_pos: (ir, ic),
    //                 new_pos: (ir + 1, ic - 1),
    //                 taken_piece: board[ir + 1][ic - 1].clone(),
    //                 check: move_results_in_check((ir, ic), (ir + 1, ic - 1), board[ir + 1][ic - 1].clone(), board, whites_turn)?,
    //                 special_rule: None
    //             }),
    //             _ => {}
    //         }
    //     }
    // }

    // return Ok(possible_moves)
}