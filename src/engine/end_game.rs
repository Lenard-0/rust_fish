use std::cmp::max;
use crate::{chess_functionality::moves::check::get_kings_position, Piece};

fn calculate_endgame_weight(board: &Vec<Vec<Option<Piece>>>) -> Result<f32, String> {
    let mut piece_count = 0;
    for row in board {
        for square in row {
            if let Some(_piece) = square {
                piece_count += 1;
            }
        }
    }

    let endgame_weight = 1.0 - (piece_count as f32 / 32.0);
    return Ok(endgame_weight)
}

pub fn force_king_to_corner_endgame_eval(
    board: &Vec<Vec<Option<Piece>>>,
    whites_turn: bool
) -> Result<i32, String> {
    let friendly_king_square = get_kings_position(board, whites_turn)?;
    let opponent_king_square = get_kings_position(board, !whites_turn)?;
    let endgame_weight = calculate_endgame_weight(board)?;

    let mut eval = 0;

    // Favour positions where opponent king has been forced away from the center(edge / corner)
    // Making it easier to deliver checkmate
    let opponent_king_distance_from_center_file = max(3 - opponent_king_square.0 as i32, opponent_king_square.0 as i32 - 4);
    let opponent_king_distance_from_center_rank = max(3 - opponent_king_square.1 as i32, opponent_king_square.1 as i32 - 4);
    let opponent_king_distance_from_center = opponent_king_distance_from_center_file + opponent_king_distance_from_center_rank;
    eval += opponent_king_distance_from_center;

    // Incentivise moving our king closer to opponent king to cut off escape routes and assist mate
    let distance_between_kings_file = (friendly_king_square.0 as i32 - opponent_king_square.0 as i32).abs();
    let distance_between_kings_rank = (friendly_king_square.1 as i32 - opponent_king_square.1 as i32).abs();
    let distance_between_kings = distance_between_kings_file + distance_between_kings_rank;
    eval += 14 - distance_between_kings;

    return Ok((eval as f32 * 10 as f32 * endgame_weight) as i32)
}

