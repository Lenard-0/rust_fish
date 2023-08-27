use crate::Piece;

pub fn calculate_pawn_moves(
    board: &Vec<Vec<Option<Piece>>>,
    ir: usize,
    ic: usize,
    whites_turn: bool,
) -> Vec<(usize, usize)> {
    let mut possible_moves = Vec::new();

    if whites_turn {
        let tile_one_up = &board[ir - 1][ic];
        if tile_one_up.is_none() {
            possible_moves.push((ir - 1, ic));
        }
        if ir == 6 { // starting rank
            let tile_two_up = &board[ir - 2][ic];
            if tile_one_up.is_none() && tile_two_up.is_none() {
                possible_moves.push((ir - 2, ic));
            }
        }

        //take left
        if ic != 0 {
            let tile = &board[ir - 1][ic - 1];
            match tile {
                Some(Piece::Black(_)) => possible_moves.push((ir - 1, ic - 1)),
                _ => {}
            }
        }
        // take right
        if ic != 7 {
            let tile = &board[ir - 1][ic + 1];
            match tile {
                Some(Piece::Black(_)) => possible_moves.push((ir - 1, ic + 1)),
                _ => {}
            }
        }
    } else {
        let tile_one_up = &board[ir + 1][ic];
        if tile_one_up.is_none() {
            possible_moves.push((ir + 1, ic));
        }
        if ir == 6 { // starting rank
            let tile_two_up = &board[ir + 2][ic];
            if tile_one_up.is_none() && tile_two_up.is_none() {
                possible_moves.push((ir + 2, ic));
            }
        }

        //take left
        if ic != 7 {
            let tile = &board[ir + 1][ic + 1];
            match tile {
                Some(Piece::Black(_)) => possible_moves.push((ir + 1, ic + 1)),
                _ => {}
            }
        }
        // take right
        if ic != 0 {
            let tile = &board[ir + 1][ic - 1];
            match tile {
                Some(Piece::Black(_)) => possible_moves.push((ir + 1, ic - 1)),
                _ => {}
            }
        }
    }

    return possible_moves
}