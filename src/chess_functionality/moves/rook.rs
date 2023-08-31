use crate::{Piece, chess_functionality::determine_appropriate_move_behaviour};

use super::{Move, move_results_in_check};


pub fn calculate_rook_moves(
    board: &mut Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Result<Vec<Move>, String> {
    let mut possible_moves = Vec::new();

    let mut i = 0; // 0 = left, 1 = right, 2 = up, 3 = down
    while i < 4 {
        let mut temp_ir = ir;
        let mut temp_ic = ic;
        loop {
            if i == 0 {
                if temp_ir as i32 - 1 < 0 {
                    break;
                }
                temp_ir -= 1; // left

            } else if i == 1 {
                temp_ir += 1; // right
            } else if i == 2 {
                if i - 1 < 0 {
                    break;
                }
                if temp_ic as i32 - 1 < 0 {
                    break;
                }
                temp_ic -= 1; // up
            } else {
                temp_ic += 1; // down
            }

            let tile = match board.get(temp_ir) {
                Some(row) => match row.get(temp_ic) {
                    Some(tile) => tile,
                    None => break
                },
                None => break
            };

            let behaviour =  determine_appropriate_move_behaviour(
                tile,
                whites_turn
            );
            if behaviour.should_add_move {
                possible_moves.push(Move {
                    current_pos: (temp_ir, temp_ic),
                    new_pos: (temp_ir, temp_ic),
                    taken_piece: behaviour.piece_taken,
                    check: move_results_in_check((ir, ic), (temp_ir ,temp_ic), tile.clone(), board, whites_turn)?,
                    special_rule: None,
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