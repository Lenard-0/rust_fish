use crate::{utils::for_each_tile, Piece};
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
    for possible_move in all_possible_enemy_moves {
        if possible_move.new_pos == king_position {
            return Ok(true)
        }
    }

    return Ok(false)
}

pub fn all_possible_moves(board: &mut Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<Vec<Move>, String> {
    let mut all_possible_moves = Vec::new();
    for_each_tile(&board.clone(), |ir, ic, tile| {
        match tile {
            Some(Piece::White(_)) if whites_turn => get_pieces_possible_moves(board, whites_turn, &mut all_possible_moves, ir, ic)?,
            Some(Piece::Black(_)) if !whites_turn => get_pieces_possible_moves(board, whites_turn, &mut all_possible_moves, ir, ic)?,
            _ => {}
        }
        Ok(())
    })?;

    Ok(all_possible_moves)
}

fn get_pieces_possible_moves(board: &mut Vec<Vec<Option<Piece>>>, whites_turn: bool, all_possible_moves: &mut Vec<Move>, ir: usize, ic: usize) -> Result<(), String> {
    let moves = calculate_possible_moves(ir, ic, board, false, whites_turn)?;
    all_possible_moves.extend(moves); // Add moves to the list
    Ok(())
}


pub fn get_kings_position(board: &Vec<Vec<Option<Piece>>>, whites_turn: bool) -> Result<(usize, usize), String> {
    let mut king_position = None;
    for_each_tile(board, |ir, ic, tile| {
        match tile {
            Some(Piece::White(crate::PieceType::King)) if whites_turn => king_position = Some((ir, ic)),
            Some(Piece::Black(crate::PieceType::King)) if !whites_turn => king_position = Some((ir, ic)),
            _ => {}
        }
        Ok(())
    })?;
    king_position.ok_or_else(|| "Tried to find king's position but king not found on board".to_string())
}