use std::{cmp::max, f64::INFINITY};

use move_thread::EngineCalculation;
use scoring::evaluate_board;

use crate::{chess_functionality::{game_over::is_checkmate, moves::{check::{all_possible_moves, king_is_checked}, king::CastleState, move_piece::{move_piece, unmove_piece}, Move}}, Piece};

pub mod move_thread;
pub mod scoring;

pub fn search_for_moves(
    depth: usize,
    mut board: Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    previous_move: &Option<Move>,
    mut castle_state: CastleState,
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
        return match is_checkmate(&mut board, whites_turn, previous_move, &mut castle_state)? {
            true => Ok(EngineCalculation {
                best_move: None,
                score: if whites_turn { -INFINITY as i32 } else { INFINITY as i32 },
                move_sequence: vec![], // No move sequence on checkmate/stalemate
            }),
            false => Ok(EngineCalculation {
                best_move: None,
                score: 0,
                move_sequence: vec![],
            }),
        };
    }

    let mut best_evaluation = -INFINITY as i32;
    let mut best_move = moves[0].clone();
    let mut best_sequence = vec![]; // Stores the best move sequence

    for m in moves {
        let taken_piece = move_piece(&m, &mut board, &mut castle_state);
        let result = search_for_moves(depth - 1, board.clone(), !whites_turn, &Some(m.clone()), castle_state.clone())?;
        let evaluation = -result.score; // Negate to switch perspective
        if evaluation > best_evaluation {
            best_evaluation = evaluation;
            best_move = m.clone();
            best_sequence = result.move_sequence.clone(); // Get the best move sequence
            best_sequence.insert(0, m.clone()); // Append current move at the start
        }
        unmove_piece(&m, &mut board, taken_piece, &mut castle_state);
    }

    Ok(EngineCalculation {
        best_move: Some(best_move),
        score: best_evaluation,
        move_sequence: best_sequence, // Return the best sequence of moves
    })
}
