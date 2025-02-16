use crate::{chess_functionality::determine_appropriate_move_behaviour, Piece};
use super::Move;

/// This function abstractly searches for each standard pieces moves (excluding pawn.)
/// Each piece searches for moves in different directions, with a different amount of to determine a "direction".
/// Then => each piece has the same move behaviour applied.
pub fn search_for_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
    directions: &[(i32, i32)], // List of (row_delta, col_delta) for movement
    max_steps: usize
) -> Result<Vec<Move>, String> {
    let mut possible_moves = Vec::new();

    for &(row_delta, col_delta) in directions {
        let mut steps_taken = 0;
        let mut temp_ir = ir as i32;
        let mut temp_ic = ic as i32;

        while steps_taken < max_steps {
            temp_ir += row_delta;
            temp_ic += col_delta;

            if temp_ir < 0 || temp_ir >= 8 || temp_ic < 0 || temp_ic >= 8 {
                break; // Stop if out of bounds
            }

            let tile = &board[temp_ir as usize][temp_ic as usize];
            let behaviour = determine_appropriate_move_behaviour(tile, whites_turn);

            if behaviour.should_add_move {
                possible_moves.push(Move::from_position(ir, ic, temp_ir, temp_ic, tile, board, whites_turn, &behaviour)?)
            }

            steps_taken += 1;

            if behaviour.should_stop_searching || steps_taken == max_steps {
                break;
            }
        }
    }

    Ok(possible_moves)
}