use crate::Piece;

use self::{rook::calculate_rook_moves, bishop::calculate_bishop_moves, knight::calculate_knight_moves, pawn::calculate_pawn_moves};

pub mod rook;
pub mod bishop;
pub mod knight;
pub mod pawn;
pub mod king;

pub fn calculate_possible_moves(
    ir: usize,
    ic: usize,
    board: &Vec<Vec<Option<Piece>>>,
    excluding_moves_that_result_in_check: bool
) -> Result<Vec<(usize, usize)>, String> {
    let tile = match board.get(ir) {
        Some(row) => match row.get(ic) {
            Some(tile) => tile,
            None => return Err(format!("No tile at position: {} {}", ir, ic))
        },
        None => return Err(format!("No tile at position: {} {}", ir, ic))
    };
    let piece = match tile{
        Some(piece) => piece.clone(),
        None => return Err(format!("No piece at position: {} {}", ir, ic))
    };

    let whites_turn: bool;
    let piece_type = match piece {
        Piece::Black(piece) => {
            whites_turn = false;
            piece
        },
        Piece::White(piece) => {
            whites_turn = true;
            piece
        },
    };

    let possible_moves_before_excluding_check = match piece_type {
        crate::PieceType::Rook => calculate_rook_moves(board, ir, ic, whites_turn),
        crate::PieceType::Knight => calculate_knight_moves(board, ir, ic, whites_turn),
        crate::PieceType::Bishop => calculate_bishop_moves(board, ir, ic, whites_turn),
        crate::PieceType::Queen => vec![
            calculate_rook_moves(board, ir, ic, whites_turn),
            calculate_bishop_moves(board, ir, ic, whites_turn)
        ].concat(),
        crate::PieceType::King => return Err("Not yet completed".to_string()),
        crate::PieceType::Pawn => calculate_pawn_moves(board, ir, ic, whites_turn),
    };

    if excluding_moves_that_result_in_check {
        let mut possible_moves = Vec::new();
        for m in possible_moves_before_excluding_check {
            if !king_is_checked(board, whites_turn)? {
                possible_moves.push(m);
            }
        }

        return Ok(possible_moves)
    } else {
        return Ok(possible_moves_before_excluding_check)
    }
}

pub fn king_is_checked(board: &Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<bool, String> {
    let mut king_position = (0, 0);
    let mut ir = 0;
    let mut ic = 0;
    for row in board {
        for tile in row {
            match tile {
                Some(Piece::White(crate::PieceType::King)) if whites_turn => king_position = (ir, ic),
                Some(Piece::Black(crate::PieceType::King)) if !whites_turn => king_position = (ir, ic),
                _ => {}
            };
            ic += 1;
        }
        ir += 1;
    }

    let all_possible_enemy_moves = all_possible_moves(board, !whites_turn)?;
    for possible_move in all_possible_enemy_moves {
        if possible_move == king_position {
            return Ok(true)
        }
    }

    return Ok(false)
}

pub fn all_possible_moves(board: &Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<Vec<(usize, usize)>, String> {
    let mut all_possible_moves = Vec::new();

    let mut ir = 0;

    for row in board {
        let mut ic = 0;
        for tile in row {
            match tile {
                Some(Piece::White(_)) if whites_turn => {
                    all_possible_moves = vec![all_possible_moves, calculate_possible_moves(ir, ic, board, false)?].concat();
                },
                Some(Piece::Black(_)) if !whites_turn => {
                    all_possible_moves = vec![all_possible_moves, calculate_possible_moves(ir, ic, board, false)?].concat();
                },
                _ => {}
            };
            ic += 1;
        }
        ir += 1;
    }

    return Ok(all_possible_moves)
}

fn move_piece(
    current_position: (usize, usize),
    new_position: (usize, usize),
    mut board: Vec<Vec<Option<Piece>>>
) -> Vec<Vec<Option<Piece>>> {
    let piece = board[current_position.0][current_position.1].clone();
    board[new_position.0][new_position.1] = piece;
    board[current_position.0][current_position.1] = None;
    return board
}