use std::f32::INFINITY;


pub mod chess_functionality;
pub mod utils;

#[derive(Clone, Debug)]
pub enum Piece {
    Black(PieceType),
    White(PieceType),
}

impl Piece {
    pub fn from_board(
        board: &mut Vec<Vec<Option<Piece>>>,
        ir: usize,
        ic: usize,
        whites_turn: bool
    ) -> Result<Piece, String> {
        match board.get(ir) {
            Some(row) => match row.get(ic) {
                Some(tile) => Piece::from_tile(tile, ir, ic, whites_turn),
                None => Err(format!("No tile at position: {} {}", ir, ic))
            },
            None => Err(format!("No tile at position: {} {}", ir, ic))
        }
    }

    pub fn from_tile(
        tile: &Option<Piece>,
        ir: usize,
        ic: usize,
        whites_turn: bool
    ) -> Result<Piece, String> {
        // also check correct color
        match tile {
            Some(Piece::Black(piece_type)) if !whites_turn => Ok(Piece::Black(piece_type.clone())),
            Some(Piece::White(piece_type)) if whites_turn => Ok(Piece::White(piece_type.clone())),
            Some(_) => Err(format!("Wrong color at position: {} {}", ir, ic)),
            None => return Err(format!("No piece at position: {} {}", ir, ic))
        }
    }
}

#[derive(Clone, Debug)]
pub enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn
}

impl PieceType {
    pub fn from_piece(piece: Piece) -> Self {
        match piece {
            Piece::Black(piece_type) => piece_type,
            Piece::White(piece_type) => piece_type
        }
    }

    pub fn directions(&self) -> &[(i32, i32)] {
        match self {
            Self::Knight => &[(-2,-1), (-2, 1), (2,-1), (2, 1), (-1, 2), (1, 2), (-1, -2), (1, -2)],
            Self::Rook => &[(0,1), (0,-1), (1,0), (-1,0)],
            Self::Bishop => &[(1,1), (-1,-1), (1,-1), (-1,1)],
            Self::Queen => &[(0,1), (0,-1), (1,0), (-1,0), (1,1), (-1,-1), (1,-1), (-1,1)],
            Self::King => &[(0,1), (0,-1), (1,0), (-1,0), (1,1), (-1,-1), (1,-1), (-1,1)],
            Self::Pawn => &[] // ignored as handled differently
        }
    }

    pub fn max_steps(&self) -> usize {
        match self {
            PieceType::Knight => 1,
            PieceType::King => 1,
            _ => INFINITY as usize
        }
    }
}