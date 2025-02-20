use std::{cmp::max, f64::INFINITY};

use move_thread::EngineCalculation;
use scoring::evaluate_board;

use crate::{chess_functionality::moves::{check::{all_possible_moves, king_is_checked}, king::CastleState, move_piece::{move_piece, unmove_piece}, Move}, Piece};

pub mod move_thread;
pub mod scoring;

pub fn search_for_moves(
    depth: usize,
    mut alpha: i32,
    beta: i32,
    mut board: &mut Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    previous_move: &Option<Move>,
    mut castle_state: &mut CastleState,
) -> Result<EngineCalculation, String> {
    if depth == 0 {
        return Ok(EngineCalculation {
            score: evaluate_board(&board, whites_turn)?,
            best_move: None,
            move_sequence: vec![], // No move sequence at depth 0
        });
    }

    let moves = all_possible_moves(&mut board, whites_turn, previous_move, &mut castle_state, false)?;

    if moves.is_empty() {
        return match king_is_checked(&mut board, whites_turn, previous_move, &mut castle_state)? {
            true => Ok(EngineCalculation {
                best_move: None,
                // score: if whites_turn { -INFINITY as i32 } else { INFINITY as i32 },
                score: if whites_turn { -100000 as i32 } else { 100000 as i32 },
                move_sequence: vec![], // No move sequence on checkmate/stalemate
            }),
            false => Ok(EngineCalculation {
                best_move: None,
                score: 0,
                move_sequence: vec![],
            }),
        };
    }

    let mut best_move = moves[0].clone();
    let mut best_sequence = vec![]; // Stores the best move sequence
    let mut best_evaluation = -INFINITY as i32;

    for m in moves {
        let taken_piece = move_piece(&m, &mut board, &mut castle_state);
        let result = search_for_moves(depth - 1, -1 * beta, -1 * alpha, &mut board, !whites_turn, &Some(m.clone()), &mut castle_state)?;
        let evaluation = -result.score; // Negate to switch perspective
        unmove_piece(&m, &mut board, taken_piece, &mut castle_state);
        if evaluation >= beta {
            return Ok(EngineCalculation { // prune
                best_move: Some(m),
                score: beta,
                move_sequence: vec![]
            });
        }
        if evaluation > best_evaluation {
            best_move = m.clone();
            best_evaluation = evaluation;
            best_sequence.insert(0, m.clone()); // Add the current move at the start of the sequence
        }
        alpha = max(alpha, evaluation);
    }

    Ok(EngineCalculation {
        best_move: Some(best_move),
        score: alpha,
        move_sequence: best_sequence, // Return the best sequence of moves
    })
}
