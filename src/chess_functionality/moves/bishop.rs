use crate::{Piece, chess_functionality::determine_appropriate_move_behaviour};


pub fn calculate_bishop_moves(
    board: &Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Vec<(usize, usize)> {
    let mut possible_moves = Vec::new();

    let mut i = 0; // 0 = left + up, 1 = right + up, 2 = left + down, 3 = right + down
    while i < 4 {
        let mut temp_ir = ir;
        let mut temp_ic = ic;
        loop {
            if i == 0 {
                temp_ir -= 1; // left + up
                temp_ic -= 1;
            } else if i == 1 {
                temp_ir -= 1; // right + up
                temp_ic += 1;
            } else if i == 2 {
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
                possible_moves.push((temp_ir, temp_ic));
            }
            if behaviour.should_stop_searching {
                break;
            }
        }

        i += 1;
    }


    return possible_moves
}