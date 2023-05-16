use crate::Piece;

pub mod moves;

pub struct MoveBehaviour {
    should_add_move: bool,
    should_stop_searching: bool
}

pub fn determine_appropriate_move_behaviour(
    tile: &Option<Piece>,
    whites_turn: bool
) -> MoveBehaviour {
    return match *tile {
        Some(Piece::White(_)) => {
            if whites_turn {
                MoveBehaviour  {
                    should_add_move: false,
                    should_stop_searching: true
                }
            } else {
                MoveBehaviour  {
                    should_add_move: true,
                    should_stop_searching: true
                }
            }
        },
        Some(Piece::Black(_)) => {
            if whites_turn {
                MoveBehaviour  {
                    should_add_move: true,
                    should_stop_searching: true
                }
            } else {
                MoveBehaviour  {
                    should_add_move: false,
                    should_stop_searching: true
                }
            }
        },
        None => MoveBehaviour  {
            should_add_move: true,
            should_stop_searching: false
        }
    }
}