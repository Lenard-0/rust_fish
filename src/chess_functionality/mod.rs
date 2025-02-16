use crate::Piece;

pub mod moves;

pub struct MoveBehaviour {
    should_add_move: bool,
    should_stop_searching: bool,
}

impl MoveBehaviour {
    pub fn same_color_piece() -> Self {
        Self  {
            should_add_move: false,
            should_stop_searching: true,
        }
    }

    pub fn different_color_piece() -> Self {
        Self {
            should_add_move: true,
            should_stop_searching: true,
        }
    }

    pub fn empty_tile() -> Self {
        Self  {
            should_add_move: true,
            should_stop_searching: false,
        }
    }
}

pub fn determine_appropriate_move_behaviour(
    tile: &Option<Piece>,
    whites_turn: bool
) -> MoveBehaviour {
    return match *tile {
        Some(Piece::White(_)) if whites_turn => MoveBehaviour::same_color_piece(),
        Some(Piece::Black(_)) if !whites_turn => MoveBehaviour::same_color_piece(),
        Some(Piece::White(_)) if !whites_turn => MoveBehaviour::different_color_piece(),
        Some(Piece::Black(_)) if whites_turn => MoveBehaviour::different_color_piece(),
        _ => MoveBehaviour::empty_tile()
    }
}