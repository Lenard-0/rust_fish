use crate::Piece;

pub fn calculate_knight_moves(
    board: &Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Vec<(usize, usize)> {
    let mut possible_moves = Vec::new();

    let mut i = 0; // 0 = left, 1 = right, 2 = up, 3 = down
    while i < 4 {
        let mut i2 = 0;
        while i2 < 2 {
            let mut temp_ic = ic;
            let mut temp_ir = ir;
            if i == 0 {
                if temp_ic as i32 - 2 < 0 {
                    i += 1;
                    continue;
                }
                temp_ic = ic - 2;
                if i2 == 0 {
                    temp_ir = ir + 1;
                } else {
                    if temp_ir as i32 - 1 < 0 {
                        i2 += 1;
                        continue;
                    }
                    temp_ir = ir - 1;
                }
            } else if i == 1 {
                temp_ic = ic + 2;
                if i2 == 0 {
                    if temp_ir as i32 - 1 < 0 {
                        i2 += 1;
                        continue;
                    }
                    temp_ir = ir - 1;
                } else {
                    temp_ir = ir + 1;
                }
            } else if i == 2 {
                if temp_ir as i32 - 2 < 0 {
                    i += 1;
                    continue;
                }
                temp_ir = ir - 2;
                if i2 == 0 {
                    if temp_ic as i32 - 1 < 0 {
                        i2 += 1;
                        continue;
                    }
                    temp_ic = ic - 1;
                } else {
                    temp_ic = ic + 1;
                }
            } else {
                temp_ir = ir + 2;
                if i2 == 0 {
                    temp_ic = ic + 1;
                } else {
                    if temp_ic as i32 - 1 < 0 {
                        i2 += 1;
                        continue;
                    }
                    temp_ic = ic - 1;
                }
            }

            match board.get(temp_ir) {
                Some(row) => match row.get(temp_ic) {
                    Some(tile) => {
                        match tile {
                            Some(Piece::White(_)) if whites_turn => {},
                            Some(Piece::Black(_)) if !whites_turn => {},
                            _ => possible_moves.push((temp_ir, temp_ic))
                        }
                    },
                    None => {}
                },
                None => {}
            };

            i2 += 1;
        }

        i += 1;
    }

    return possible_moves
}