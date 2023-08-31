
use crate::Piece;

use super::{Move, move_results_in_check};


pub fn calculate_king_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Result<Vec<Move>, String> {
    let mut possible_moves = Vec::new();

    // up
    if ir as i32 - 1 >= 0 {
        // up
        match board.get(ir - 1) {
            Some(row) => match row.get(ic) {
                Some(tile) => match tile {
                    Some(Piece::White(_)) if whites_turn => {},
                    Some(Piece::Black(_)) if !whites_turn => {},
                    _ => possible_moves.push(Move {
                        current_pos: (ir, ic),
                        new_pos: (ir - 1, ic),
                        taken_piece: board[ir - 1][ic].clone(),
                        check: move_results_in_check((ir, ic), (ir - 1, ic), board[ir - 1][ic].clone(), board, whites_turn)?,
                        special_rule: None
                    })
                },
                None => {}
            },
            None => {}
        };

        // up - left
        if ic as i32 - 1 > 0 {
            match board.get(ir - 1) {
                Some(row) => match row.get(ic - 1) {
                    Some(tile) => match tile {
                        Some(Piece::White(_)) if whites_turn => {},
                        Some(Piece::Black(_)) if !whites_turn => {},
                        _ => possible_moves.push(Move {
                            current_pos: (ir, ic),
                            new_pos: (ir - 1, ic - 1),
                            taken_piece: board[ir - 1][ic - 1].clone(),
                            check: move_results_in_check((ir, ic), (ir - 1, ic - 1), board[ir - 1][ic - 1].clone(), board, whites_turn)?,
                            special_rule: None
                        })
                    },
                    None => {}
                },
                None => {}
            };
        }

        //up - right
        match board.get(ir - 1) {
            Some(row) => match row.get(ic + 1) {
                Some(tile) => match tile {
                    Some(Piece::White(_)) if whites_turn => {},
                    Some(Piece::Black(_)) if !whites_turn => {},
                    _ => possible_moves.push(Move {
                        current_pos: (ir, ic),
                        new_pos: (ir - 1, ic + 1),
                        taken_piece: board[ir - 1][ic + 1].clone(),
                        check: move_results_in_check((ir, ic), (ir - 1, ic + 1), board[ir - 1][ic + 1].clone(), board, whites_turn)?,
                        special_rule: None
                    })
                },
                None => {}
            },
            None => {}
        };
    }

    // left
    if ic as i32 - 1 > 0 {
        match board.get(ir) {
            Some(row) => match row.get(ic - 1) {
                Some(tile) => match tile {
                    Some(Piece::White(_)) if whites_turn => {},
                    Some(Piece::Black(_)) if !whites_turn => {},
                    _ => possible_moves.push(Move {
                        current_pos: (ir, ic),
                        new_pos: (ir, ic - 1),
                        taken_piece: board[ir][ic - 1].clone(),
                        check: move_results_in_check((ir, ic), (ir, ic - 1), board[ir][ic - 1].clone(), board, whites_turn)?,
                        special_rule: None
                    })
                },
                None => {}
            },
            None => {}
        };
    }

    // right
    match board.get(ir) {
        Some(row) => match row.get(ic + 1) {
            Some(tile) => match tile {
                Some(Piece::White(_)) if whites_turn => {},
                Some(Piece::Black(_)) if !whites_turn => {},
                _ => possible_moves.push(Move {
                    current_pos: (ir, ic),
                    new_pos: (ir, ic + 1),
                    taken_piece: board[ir][ic + 1].clone(),
                    check: move_results_in_check((ir, ic), (ir, ic + 1), board[ir][ic + 1].clone(), board, whites_turn)?,
                    special_rule: None
                })
            },
            None => {}
        },
        None => {}
    };

    // down - left
    if ic as i32 - 1 > 0 {
        match board.get(ir + 1) {
            Some(row) => match row.get(ic - 1) {
                Some(tile) => match tile {
                    Some(Piece::White(_)) if whites_turn => {},
                    Some(Piece::Black(_)) if !whites_turn => {},
                    _ => possible_moves.push(Move {
                        current_pos: (ir, ic),
                        new_pos: (ir + 1, ic - 1),
                        taken_piece: board[ir + 1][ic - 1].clone(),
                        check: move_results_in_check((ir, ic), (ir + 1, ic - 1), board[ir + 1][ic - 1].clone(), board, whites_turn)?,
                        special_rule: None
                    })
                },
                None => {}
            },
            None => {}
        };
    }

    // down
    match board.get(ir + 1) {
        Some(row) => match row.get(ic) {
            Some(tile) => match tile {
                Some(Piece::White(_)) if whites_turn => {},
                Some(Piece::Black(_)) if !whites_turn => {},
                _ => possible_moves.push(Move {
                    current_pos: (ir, ic),
                    new_pos: (ir + 1, ic),
                    taken_piece: board[ir + 1][ic].clone(),
                    check: move_results_in_check((ir, ic), (ir + 1, ic), board[ir + 1][ic].clone(), board, whites_turn)?,
                    special_rule: None
                })
            },
            None => {}
        },
        None => {}
    };

    // down - right
    match board.get(ir + 1) {
        Some(row) => match row.get(ic + 1) {
            Some(tile) => match tile {
                Some(Piece::White(_)) if whites_turn => {},
                Some(Piece::Black(_)) if !whites_turn => {},
                _ => possible_moves.push(Move {
                    current_pos: (ir, ic),
                    new_pos: (ir + 1, ic + 1),
                    taken_piece: board[ir + 1][ic + 1].clone(),
                    check: move_results_in_check((ir, ic), (ir + 1, ic + 1), board[ir + 1][ic + 1].clone(), board, whites_turn)?,
                    special_rule: None
                })
            },
            None => {}
        },
        None => {}
    };

    return Ok(possible_moves)
}