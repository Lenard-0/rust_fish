
#[cfg(test)]
mod tests {
    use rust_fish_chess_engine::{utils::{decode_board, encode_board}, Piece, PieceType};


        #[test]
        fn test_encode_decode() {
            // Here is the sample board from your example.
            let board = vec![
                vec![
                    Some(Piece::Black(PieceType::Rook)),
                    None,
                    Some(Piece::Black(PieceType::Bishop)),
                    Some(Piece::Black(PieceType::Queen)),
                    Some(Piece::Black(PieceType::King)),
                    Some(Piece::Black(PieceType::Bishop)),
                    None,
                    Some(Piece::Black(PieceType::Rook)),
                ],
                vec![
                    None,
                    Some(Piece::Black(PieceType::Pawn)),
                    Some(Piece::Black(PieceType::Pawn)),
                    None,
                    None,
                    None,
                    Some(Piece::Black(PieceType::Pawn)),
                    None,
                ],
                vec![
                    Some(Piece::Black(PieceType::Pawn)),
                    None,
                    Some(Piece::Black(PieceType::Knight)),
                    None,
                    Some(Piece::White(PieceType::Rook)),
                    Some(Piece::Black(PieceType::Knight)),
                    None,
                    Some(Piece::Black(PieceType::Pawn)),
                ],
                vec![None, None, None, None, None, None, None, None],
                vec![
                    None,
                    None,
                    Some(Piece::White(PieceType::Bishop)),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                vec![
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::White(PieceType::Knight)),
                    None,
                    None,
                ],
                vec![
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    None,
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                ],
                vec![
                    Some(Piece::White(PieceType::Rook)),
                    Some(Piece::White(PieceType::Knight)),
                    Some(Piece::White(PieceType::Bishop)),
                    Some(Piece::White(PieceType::Queen)),
                    None,
                    None,
                    Some(Piece::White(PieceType::King)),
                    None,
                ],
            ];

            // Encode the board
            let key = encode_board(&board);
            println!("Encoded key: {}", key);

            // Decode it back
            let decoded_board = decode_board(&key).expect("Decoding failed");

            // The decoded board should be identical to the original
            assert_eq!(board, decoded_board);
        }
}