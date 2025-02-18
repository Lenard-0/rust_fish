use move_thread::MoveThread;
use scoring::determine_move_score;

use crate::{chess_functionality::moves::{calculate_possible_moves, king::CastleState, move_piece::move_piece, Move}, utils::{encode_board, for_each_tile}, Piece};

pub mod move_thread;
pub mod scoring;

pub fn rust_fish_choose_move(
    current_step: usize,
    steps: usize,
    mut current_move_thread: Option<MoveThread>,
    mut move_threads: Vec<MoveThread>,
    board: Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    castle_state: &mut CastleState,
    start_turn_is_white: bool,
) -> Result<Vec<MoveThread>, String> {
    if steps == current_step {
        move_threads.push(match current_move_thread {
            Some(m) => m,
            None => return Err("No move thread".to_string())
        });
        return Ok(move_threads)
    }

    for_each_tile(&board.clone(), |ir, ic, tile| {
        match tile {
            Some(Piece::White(_)) if whites_turn => {
                move_threads = vec![move_threads.clone(), make_all_possible_moves_and_add_to_threads(
                    ir,
                    ic,
                    current_step,
                    steps,
                    current_move_thread.clone(),
                    move_threads.clone(),
                    board.clone(),
                    whites_turn,
                    start_turn_is_white,
                    castle_state
                )?].concat();
            },
            Some(Piece::Black(_)) if !whites_turn => {
                move_threads = vec![move_threads.clone(), make_all_possible_moves_and_add_to_threads(
                    ir,
                    ic,
                    current_step,
                    steps,
                    current_move_thread.clone(),
                    move_threads.clone(),
                    board.clone(),
                    whites_turn,
                    start_turn_is_white,
                    castle_state
                )?].concat();
            },
            _ => {}
        }
        Ok(())
    })?;

    return Ok(move_threads)

}

fn make_all_possible_moves_and_add_to_threads(
    ir: usize,
    ic: usize,
    current_step: usize,
    max_steps: usize,
    mut current_move_thread: Option<MoveThread>,
    mut move_threads: Vec<MoveThread>,
    board: Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    start_turn_is_white: bool,
    castle_state: &mut CastleState,
) -> Result<Vec<MoveThread>, String> {
    let all_possible_moves = calculate_possible_moves(
        ir,
        ic,
        &mut board.clone(),
        false,
        whites_turn,
        &None,
        castle_state,
        false
    )?;

    for m in all_possible_moves {
        let mut distinct_board = board.clone();
        let mut distinct_castle_state = castle_state.clone();
        let taken_piece = move_piece(&m, &mut distinct_board, &mut distinct_castle_state);
        match current_move_thread {
            None => {
                let new_move_thread = MoveThread {
                    moves: vec![m],
                    current_score: determine_move_score(
                        taken_piece,
                        (whites_turn && start_turn_is_white) || (!start_turn_is_white && !whites_turn),
                        &mut distinct_board,
                        whites_turn,
                        &None,
                        castle_state
                    )?,
                    current_board_encoded: encode_board(&distinct_board)
                };
                move_threads.push(new_move_thread.clone());
                // let threads = rust_fish_choose_move(
                //     current_step + 1,
                //     match start_turn_is_white && whites_turn || !start_turn_is_white && !whites_turn {
                //         true => current_step + 1,
                //         false => current_step
                //     },
                //      Some(new_move_thread),
                //      move_threads.clone(),
                //     distinct_board,
                //     !whites_turn,
                //     &mut distinct_castle_state,
                //     false
                // )?;
            },
            Some(ref mut current_move_thread) => {
                // let mut continued_move_thread = current_move_thread.clone();
                // continued_move_thread.moves.push(m);
                // current_move_thread.current_score += determine_move_score(taken_piece)?;
                // current_move_thread.current_board_encoded = encode_board(&distinct_board);
                // let thread = rust_fish_choose_move(
                //     current_step + 1,
                //     match start_turn_is_white && whites_turn || !start_turn_is_white && !whites_turn {
                //         true => current_step + 1,
                //         false => current_step
                //     },
                //     Some(continued_move_thread),
                //     move_threads.clone(),
                //     distinct_board,
                //     !whites_turn,
                //     &mut distinct_castle_state,
                //     false
                // )?;
                // move_threads = vec![move_threads.clone(), thread].concat();
            }
        };
    }

    return Ok(move_threads)
}

pub fn get_top_thread(threads: Vec<MoveThread>) -> Result<MoveThread, String> {
    let mut top_thread = threads[0].clone();
    for t in threads {
        if t.current_score > top_thread.current_score {
            top_thread = t;
        }
    }
    return Ok(top_thread)
}