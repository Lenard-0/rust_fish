use std::{cmp::max, f64::INFINITY};
use end_game::force_king_to_corner_endgame_eval;
use move_thread::EngineCalculation;
use ordering::order_moves;
use crate::{chess_functionality::moves::{check::{all_possible_moves, king_is_checked}, king::CastleState, move_piece::{move_piece, unmove_piece}, Move}, Piece};
use crate::engine::search_captures::search_captures;

pub mod move_thread;
pub mod scoring;
pub mod ordering;
pub mod search_captures;
pub mod end_game;
pub mod piece_map;

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
        return search_captures(alpha, beta, board, whites_turn, previous_move, &mut castle_state);
    }

    let moves = all_possible_moves(&mut board, whites_turn, previous_move, &mut castle_state, false, true)?;

    if moves.is_empty() {
        return match king_is_checked(&mut board, whites_turn, previous_move, &mut castle_state) {
            Ok(true) | Err(_) => Ok(EngineCalculation {
                best_move: None,
                // score: if whites_turn { -INFINITY as i32 } else { INFINITY as i32 },
                score: -10000,
                // move_sequence: vec![], // No move sequence on checkmate/stalemate
            }),
            Ok(false) => Ok(EngineCalculation {
                best_move: None,
                score: 0,
                // move_sequence: vec![],
            }),
        };
    }

    let mut best_move = moves[0].clone();
    let mut best_sequence = vec![]; // Stores the best move sequence
    let mut best_evaluation = -INFINITY as i32;

    let ordered_moves = order_moves(moves, board, whites_turn)?;
    for m in ordered_moves {
        let taken_piece = move_piece(&m, &mut board, &mut castle_state.clone());
        let result = search_for_moves(
            depth - 1,
            -1 * beta,
            -1 * alpha,
            &mut board,
            !whites_turn,
            &Some(m.clone()),
            &mut castle_state
        )?;
        let evaluation = -result.score - force_king_to_corner_endgame_eval(board, !whites_turn)?; // Negate to switch perspective
        unmove_piece(&m, &mut board, taken_piece)?;
        if evaluation >= beta {
            return Ok(EngineCalculation { // prune
                best_move: Some(m),
                score: beta,
                // move_sequence: vec![]
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
        // move_sequence: best_sequence, // Return the best sequence of moves
    })
}


