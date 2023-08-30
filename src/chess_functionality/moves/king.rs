
use crate::{Piece, chess_functionality::determine_appropriate_move_behaviour};


pub fn calculate_king_moves(
    board: &Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Vec<(usize, usize)> {
    let mut possible_moves = Vec::new();


    // up
    if ir as i32 - 1 >= 0 {
        // up
        match board.get(ir - 1) {
            Some(row) => match row.get(ic) {
                Some(tile) => match tile {
                    Some(Piece::White(_)) if whites_turn => {},
                    Some(Piece::Black(_)) if !whites_turn => {},
                    _ => possible_moves.push((ir - 1, ic))
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
                        _ => possible_moves.push((ir - 1, ic - 1))
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
                    _ => possible_moves.push((ir - 1, ic + 1))
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
                    _ => possible_moves.push((ir, ic - 1))
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
                _ => possible_moves.push((ir, ic + 1))
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
                    _ => possible_moves.push((ir + 1, ic - 1))
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
                _ => possible_moves.push((ir + 1, ic))
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
                _ => possible_moves.push((ir + 1, ic + 1))
            },
            None => {}
        },
        None => {}
    };

    return possible_moves
}