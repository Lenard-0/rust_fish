use crate::{Piece, chess_functionality::determine_appropriate_move_behaviour};

use super::{Move, move_results_in_check};


pub fn calculate_bishop_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Result<Vec<Move>, String> {
    let mut possible_moves = Vec::new();

    let mut i = 0; // 0 = left + up, 1 = right + up, 2 = left + down, 3 = right + down
    while i < 4 {
        let mut temp_ir = ir;
        let mut temp_ic = ic;
        loop {
            if i == 0 {
                if temp_ir as i32 - 1 < 0 || temp_ic as i32 - 1 < 0 {
                    break;
                }
                temp_ir -= 1; // left + up
                temp_ic -= 1;
            } else if i == 1 {
                if temp_ir as i32 - 1 < 0  {
                    break;
                }
                temp_ir -= 1; // right + up
                temp_ic += 1;
            } else if i == 2 {
                if temp_ic as i32 - 1 < 0 {
                    break;
                }
                temp_ir += 1; // left + down
                temp_ic -= 1;
            } else {
                temp_ir += 1; // right + down
                temp_ic += 1;
            }

            let tile = match board.get(temp_ir) {
                Some(row) => match row.get(temp_ic) {
                    Some(tile) => tile,
                    None => break
                },
                None => break
            };


            let behaviour =  determine_appropriate_move_behaviour(
                &tile,
                whites_turn
            );

            if behaviour.should_add_move {
                possible_moves.push(Move {
                    current_pos: (ir, ic),
                    new_pos: (temp_ir, temp_ic),
                    taken_piece: behaviour.piece_taken.clone(),
                    check: move_results_in_check((ir, ic), (temp_ir, temp_ic), behaviour.piece_taken, board, whites_turn)?,
                    special_rule: None
                });
            }
            if behaviour.should_stop_searching {
                break;
            }
        }

        i += 1;
    }


    return Ok(possible_moves)
}