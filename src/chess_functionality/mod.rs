use crate::Piece;

pub mod moves;

pub struct MoveBehaviour {
    should_add_move: bool,
    should_stop_searching: bool,
    piece_taken: Option<Piece>
}

impl MoveBehaviour {
    pub fn same_color_piece() -> Self {
        Self  {
            should_add_move: false,
            should_stop_searching: true,
            piece_taken: None
        }
    }

    pub fn different_color_piece(tile: &Option<Piece>) -> Self {
        Self {
            should_add_move: true,
            should_stop_searching: true,
            piece_taken: tile.clone()
        }
    }

    pub fn empty_tile() -> Self {
        Self  {
            should_add_move: true,
            should_stop_searching: false,
            piece_taken: None
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
        Some(Piece::White(_)) if !whites_turn => MoveBehaviour::different_color_piece(tile),
        Some(Piece::Black(_)) if whites_turn => MoveBehaviour::different_color_piece(tile),
        _ => MoveBehaviour::empty_tile()
    }
}