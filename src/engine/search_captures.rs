use std::cmp::max;

use crate::{chess_functionality::moves::{check::all_possible_moves, king::CastleState, move_piece::{move_piece, unmove_piece}, Move}, Piece};
use super::{move_thread::EngineCalculation, ordering::order_moves, scoring::evaluate_board};



pub fn search_captures(
    mut alpha: i32,
    beta: i32,
    mut board: &mut Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    previous_move: &Option<Move>,
    castle_state: &mut CastleState,
) -> Result<EngineCalculation, String> {
    // Captures typically aren't forced so see what the eval is before making a capture.
    // Otherwise, if only bad captures are available, it will be evaluated as bad.
    // Even if good non-capture moves are available.

    let mut evaluation = evaluate_board(&board, whites_turn)?;
    if evaluation >= beta {
        return Ok(EngineCalculation {
            best_move: None,
            score: beta,
            // move_sequence: vec![]
        });
    }

    alpha = max(alpha, evaluation);

    let possible_capture_moves = all_possible_moves(
        board,
        whites_turn,
        previous_move,
        castle_state,
        true,
        false
    )?;

    let ordered_possible_captures = order_moves(possible_capture_moves, board, whites_turn)?;

    for m in ordered_possible_captures {
        let taken_piece = move_piece(&m, &mut board, &mut castle_state.clone());
        evaluation = -1 * evaluate_board(&board, whites_turn)?;
        unmove_piece(&m, &mut board, taken_piece)?;

        if evaluation >= beta {
            return Ok(EngineCalculation {
                best_move: Some(m),
                score: beta,
                // move_sequence: vec![]
            });
        }
        if evaluation > alpha {
            alpha = evaluation;
        }
    }

    Ok(EngineCalculation {
        best_move: None,
        score: alpha,
        // move_sequence: vec![]
    })
}