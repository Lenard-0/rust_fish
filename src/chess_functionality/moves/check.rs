use crate::Piece;
use super::{calculate_possible_moves, move_piece, unmove_piece, Move};

pub fn move_results_in_check(
    move_to_check: Move,
    board: &mut Vec<Vec<Option<Piece>>>,
    whites_turn: bool
) -> Result<bool, String> {
    let taken_piece = move_piece(&move_to_check, board);
    let check = king_is_checked(board, whites_turn)?;
    unmove_piece(&move_to_check, board, taken_piece);
    return Ok(check)
}

fn king_is_checked(board: &mut Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<bool, String> {
    let king_position = get_kings_position(board, whites_turn)?;
    let all_possible_enemy_moves = all_possible_moves(board, !whites_turn)?;
    println!("all_possible_enemy_moves: {:#?}", all_possible_enemy_moves);
    for possible_move in all_possible_enemy_moves {
        if possible_move.new_pos == king_position {
            return Ok(true)
        }
    }

    return Ok(false)
}

fn all_possible_moves(board: &mut Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<Vec<Move>, String> {
    let mut all_possible_moves = Vec::new();

    let mut ir = 0;

    for row in board.clone() {
        let mut ic = 0;
        for tile in row {
            match tile {
                Some(Piece::White(_)) if whites_turn => {
                    all_possible_moves = vec![all_possible_moves, calculate_possible_moves(ir, ic, board, false, whites_turn)?].concat();
                },
                Some(Piece::Black(_)) if !whites_turn => {
                    all_possible_moves = vec![all_possible_moves, calculate_possible_moves(ir, ic, board, false, whites_turn)?].concat();
                },
                _ => {}
            };
            ic += 1;
        }
        ir += 1;
    }

    return Ok(all_possible_moves)
}

fn get_kings_position(board: &mut Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<(usize, usize), String> {
    let mut ir = 0;
    let mut ic = 0;
    for row in board.iter() {
        for tile in row {
            match tile {
                Some(Piece::White(crate::PieceType::King)) if whites_turn => return Ok((ir, ic)),
                Some(Piece::Black(crate::PieceType::King)) if !whites_turn => return Ok((ir, ic)),
                _ => {}
            };
            ic += 1;
        }
        ir += 1;
    }

    return Err("Tried to find king's position but king not found on board".to_string())
}