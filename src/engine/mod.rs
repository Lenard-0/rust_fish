use move_thread::MoveThread;
use scoring::determine_move_score;

use crate::{chess_functionality::moves::{calculate_possible_moves, king::CastleState, move_piece::move_piece, Move}, utils::for_each_tile, Piece};

pub mod move_thread;
pub mod scoring;

pub fn rust_fish_choose_move(
    current_step: usize,
    steps: usize,
    move_threads: Vec<MoveThread>,
    board: Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    castle_state: &mut CastleState,
    start_turn_is_white: bool,
) -> Result<Vec<MoveThread>, String> {
    unimplemented!()
    // while current_step < steps {
    //     for_each_tile(&board.clone(), |ir, ic, tile| {
    //         match tile {
    //             Some(Piece::White(_)) if whites_turn => {

    //             },
    //             Some(Piece::Black(_)) if !whites_turn => {
    //                 let all_possible_moves = calculate_possible_moves(
    //                     ir,
    //                     ic,
    //                     &mut board.clone(),
    //                     false,
    //                     whites_turn,
    //                     &None,
    //                     castle_state,
    //                     false
    //                 )?;
    //             },
    //             _ => {}
    //         }
    //         Ok(())
    //     })?;
    // }
}

async fn make_all_possible_moves_and_add_to_threads(
    ir: usize,
    ic: usize,
    current_step: usize,
    move_threads: &mut Vec<MoveThread>,
    board: Vec<Vec<Option<Piece>>>,
    whites_turn: bool,
    castle_state: &mut CastleState,
) -> Result<(), String> {
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
        if current_step == 0 {
            let move_thread = MoveThread {
                moves: vec![m],
                current_score: determine_move_score(taken_piece)?,
                current_board: distinct_board
            };
            move_threads.push(move_thread);
        }
    }

    Ok(())
}

