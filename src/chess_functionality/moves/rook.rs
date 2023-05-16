use crate::{Piece, chess_functionality::determine_appropriate_move_behaviour};


pub fn calculate_rook_moves(
    board: &Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Vec<(usize, usize)> {
    let mut possible_moves = Vec::new();

    let mut i = 0; // 0 = left, 1 = right, 2 = up, 3 = down
    while i < 4 {
        let mut temp_ir = ir;
        let mut temp_ic = ic;
        loop {
            if i == 0 {
                temp_ir -= 1; // left
            } else if i == 1 {
                temp_ir += 1; // right
            } else if i == 2 {
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