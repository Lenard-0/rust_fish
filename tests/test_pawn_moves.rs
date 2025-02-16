
// #[cfg(test)]
// mod tests {
//     use rust_fish::{chess_functionality::moves::calculate_possible_moves, Piece, PieceType};

//     #[test]
//     fn pawn_moves_one_step_forward() {
//         let mut board = vec![vec![None; 8]; 8];
//         board[4][4] = Some(Piece::White(PieceType::Pawn));

//         let expected_moves = vec![(3, 4)]; // One step forward

//         let possible_moves = calculate_possible_moves(4, 4, &mut board, true).unwrap();
//         assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
//         for m in possible_moves {
//             assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
//         }
//     }

//     #[test]
//     fn pawn_moves_two_steps_forward_from_start() {
//         let mut board = vec![vec![None; 8]; 8];
//         board[6][4] = Some(Piece::White(PieceType::Pawn));

//         let expected_moves = vec![(5, 4), (4, 4)]; // One or two steps forward

//         let possible_moves = calculate_possible_moves(6, 4, &mut board, true).unwrap();
//         assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
//         for m in possible_moves {
//             assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
//         }
//     }

//     #[test]
//     fn pawn_cannot_move_if_blocked() {
//         let mut board = vec![vec![None; 8]; 8];
//         board[4][4] = Some(Piece::White(PieceType::Pawn));
//         board[3][4] = Some(Piece::Black(PieceType::Pawn)); // Blocking forward move

//         let possible_moves = calculate_possible_moves(4, 4, &mut board, true).unwrap();
//         assert!(possible_moves.is_empty(), "Pawn should not move when blocked");
//     }

//     #[test]
//     fn pawn_can_capture_diagonally() {
//         let mut board = vec![vec![None; 8]; 8];
//         board[4][4] = Some(Piece::White(PieceType::Pawn));
//         board[3][3] = Some(Piece::Black(PieceType::Pawn)); // Capturable
//         board[3][5] = Some(Piece::Black(PieceType::Pawn)); // Capturable

//         let expected_moves = vec![(3, 4), (3, 3), (3, 5)]; // Forward + Diagonal captures

//         let possible_moves = calculate_possible_moves(4, 4, &mut board, true).unwrap();
//         assert_eq!(possible_moves.len(), expected_moves.len(), "Expected and actual moves differ in count");
//         for m in possible_moves {
//             assert!(expected_moves.contains(&m.new_pos), "Unexpected move: {:?}", m.new_pos);
//         }
//     }

//     #[test]
//     fn pawn_cannot_capture_forward() {
//         let mut board = vec![vec![None; 8]; 8];
//         board[4][4] = Some(Piece::White(PieceType::Pawn));
//         board[3][4] = Some(Piece::Black(PieceType::Pawn)); // In front

//         let expected_moves: Vec<(usize, usize)> = vec![]; // No valid moves

//         let possible_moves = calculate_possible_moves(4, 4, &mut board, true).unwrap();
//         assert_eq!(possible_moves.len(), expected_moves.len(), "Pawn should not capture forward");
//     }

// }