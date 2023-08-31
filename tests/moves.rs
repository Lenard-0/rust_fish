use rust_fish::{Piece, PieceType, chess_functionality::moves::{calculate_possible_moves, Move}};


#[test]
fn can_move_rook() {
    let mut board = vec![
        vec![Some(Piece::Black(PieceType::Rook)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Queen)), Some(Piece::Black(PieceType::King)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Rook))],
        vec![Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn))],
        vec![None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None],
        vec![None, None, Some(Piece::White(PieceType::Rook)), None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None],
        vec![Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn))],
        vec![Some(Piece::White(PieceType::Rook)), Some(Piece::White(PieceType::Knight)), Some(Piece::White(PieceType::Bishop)), Some(Piece::White(PieceType::Queen)), Some(Piece::White(PieceType::King)), Some(Piece::White(PieceType::Bishop)), Some(Piece::White(PieceType::Knight)), Some(Piece::White(PieceType::Rook))],
    ];

    let possible_moves = calculate_possible_moves(
        4,
        2,
        &mut board,
        true
    ).unwrap();

    let expected_moves = vec![
        (4, 0), (4, 1), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7),
        (5, 2), (3, 2), (2, 2), (1, 2)
    ];

    if possible_moves.len() != expected_moves.len() {
        panic!("expected and possible moves are not same length")
    }

    for m in possible_moves {
        if !expected_moves.contains(&m.new_pos) {
            panic!("unexpected move")
        }
    }
}

#[test]
fn can_move_knight() {
    let mut board = vec![
        vec![Some(Piece::Black(PieceType::Rook)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Queen)), Some(Piece::Black(PieceType::King)), Some(Piece::Black(PieceType::Bishop)), Some(Piece::Black(PieceType::Knight)), Some(Piece::Black(PieceType::Rook))],
        vec![Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn)), Some(Piece::Black(PieceType::Pawn))],
        vec![None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None],
        vec![None, None, Some(Piece::White(PieceType::Knight)), None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None],
        vec![Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn)), Some(Piece::White(PieceType::Pawn))],
        vec![Some(Piece::White(PieceType::Rook)), Some(Piece::White(PieceType::Knight)), Some(Piece::White(PieceType::Bishop)), Some(Piece::White(PieceType::Queen)), Some(Piece::White(PieceType::King)), Some(Piece::White(PieceType::Bishop)), Some(Piece::White(PieceType::Knight)), Some(Piece::White(PieceType::Rook))],
    ];

    let possible_moves = calculate_possible_moves(
        4,
        2,
        &mut board,
        true
    ).unwrap();

    let expected_moves = vec![
        (2, 1), (2, 3), (3, 4), (5, 4), (5, 0), (3, 0)
    ];

    if possible_moves.len() != expected_moves.len() {
        panic!("expected and possible moves are not same length")
    }

    for m in possible_moves {
        if !expected_moves.contains(&m.new_pos) {
            panic!("unexpected move")
        }
    }
}