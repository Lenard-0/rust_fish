use crate::{Piece, PieceType};


pub fn for_each_tile<T, F>(board: &Vec<Vec<Option<Piece>>>, mut f: F) -> Result<Vec<T>, String>
where
    F: FnMut(usize, usize, &Option<Piece>) -> Result<T, String>,
{
    let mut result = Vec::new();
    let mut ir = 0;

    for row in board.iter() {
        let mut ic = 0;
        for tile in row.iter() {
            let res = f(ir, ic, tile)?;
            result.push(res);
            ic += 1;
        }
        ir += 1;
    }

    Ok(result)
}

pub fn encode_board(board: &[Vec<Option<Piece>>]) -> String {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|square| match square {
                    None => '.',
                    Some(piece) => match piece {
                        Piece::White(pt) => match pt {
                            PieceType::Pawn => 'P',
                            PieceType::Knight => 'N',
                            PieceType::Bishop => 'B',
                            PieceType::Rook => 'R',
                            PieceType::Queen => 'Q',
                            PieceType::King => 'K',
                        },
                        Piece::Black(pt) => match pt {
                            PieceType::Pawn => 'p',
                            PieceType::Knight => 'n',
                            PieceType::Bishop => 'b',
                            PieceType::Rook => 'r',
                            PieceType::Queen => 'q',
                            PieceType::King => 'k',
                        },
                    },
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("/")
}

pub fn decode_board(key: &str) -> Result<Vec<Vec<Option<Piece>>>, String> {
    let rows: Vec<&str> = key.split('/').collect();
    if rows.len() != 8 {
        return Err(format!(
            "Invalid board: expected 8 rows but found {}",
            rows.len()
        ));
    }

    let mut board = Vec::with_capacity(8);
    for (i, row) in rows.iter().enumerate() {
        if row.chars().count() != 8 {
            return Err(format!(
                "Invalid board: row {} expected 8 columns but found {}",
                i,
                row.chars().count()
            ));
        }

        let mut row_vec = Vec::with_capacity(8);
        for ch in row.chars() {
            let square = match ch {
                '.' => None,
                'P' => Some(Piece::White(PieceType::Pawn)),
                'N' => Some(Piece::White(PieceType::Knight)),
                'B' => Some(Piece::White(PieceType::Bishop)),
                'R' => Some(Piece::White(PieceType::Rook)),
                'Q' => Some(Piece::White(PieceType::Queen)),
                'K' => Some(Piece::White(PieceType::King)),
                'p' => Some(Piece::Black(PieceType::Pawn)),
                'n' => Some(Piece::Black(PieceType::Knight)),
                'b' => Some(Piece::Black(PieceType::Bishop)),
                'r' => Some(Piece::Black(PieceType::Rook)),
                'q' => Some(Piece::Black(PieceType::Queen)),
                'k' => Some(Piece::Black(PieceType::King)),
                _ => return Err(format!("Invalid character in board: {}", ch)),
            };
            row_vec.push(square);
        }
        board.push(row_vec);
    }
    Ok(board)
}