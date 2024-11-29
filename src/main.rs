mod enums;
use enums::{Board, Color, Piece, Position};

fn main() {
    let board = Board {
        pieces: [
            [
                Some(Piece::Rook(Color::White)),
                Some(Piece::Knight(Color::White)),
                Some(Piece::Bishop(Color::White)),
                Some(Piece::Queen(Color::White)),
                Some(Piece::King(Color::White)),
                Some(Piece::Bishop(Color::White)),
                Some(Piece::Knight(Color::White)),
                Some(Piece::Rook(Color::White)),
            ],
            [
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
            ],
            [
                Some(Piece::Rook(Color::Black)),
                Some(Piece::Knight(Color::Black)),
                Some(Piece::Bishop(Color::Black)),
                Some(Piece::Queen(Color::Black)),
                Some(Piece::King(Color::Black)),
                Some(Piece::Bishop(Color::Black)),
                Some(Piece::Knight(Color::Black)),
                Some(Piece::Rook(Color::Black)),
            ],
        ],
    };

    let board = Board {
        pieces: [
            [
                Some(Piece::Rook(Color::White)),
                Some(Piece::Knight(Color::White)),
                Some(Piece::Bishop(Color::White)),
                Some(Piece::Queen(Color::White)),
                Some(Piece::King(Color::White)),
                Some(Piece::Bishop(Color::White)),
                Some(Piece::Knight(Color::White)),
                Some(Piece::Rook(Color::White)),
            ],
            [
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
                Some(Piece::Pawn(Color::White)),
            ],
            [
                Some(Piece::Pawn(Color::Black)),
                None,
                Some(Piece::Pawn(Color::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
                Some(Piece::Pawn(Color::Black)),
            ],
            [
                Some(Piece::Rook(Color::Black)),
                Some(Piece::Knight(Color::Black)),
                Some(Piece::Bishop(Color::Black)),
                Some(Piece::Queen(Color::Black)),
                Some(Piece::King(Color::Black)),
                Some(Piece::Bishop(Color::Black)),
                Some(Piece::Knight(Color::Black)),
                Some(Piece::Rook(Color::Black)),
            ],
        ],
    };

    println!("{}", board);

    let my_piece = &board.pieces[1][1];
    println!("piece: {:?}", my_piece);

    let result = valid_moves_of_piece(&board, 1, 1);
    println!("Valid moves for Pawn in Row: {}, Col: {}:", 1, 1);
    println!("size of result: {}", result.len());
    for e in result.iter() {
        println!("{:#?}", e);
    }

    // let piece = Piece::Rook(Color::White);
    // println!("{}", piece);
    // let piece = Piece::Rook(Color::Black);
    // println!("{}", piece);
}

// given a row, col coordinate, gives me the row and cols that it can move to
// returns a vector of the move set
fn valid_moves_of_piece(board: &Board, row: usize, col: usize) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::new();

    match &board.pieces[row][col] {
        None => result,
        Some(piece) => match piece {
            Piece::Pawn(color) => {
                let vertical_move_offsets = [[1, 0], [2, 0]];
                for vertical_move_offset in vertical_move_offsets.iter() {
                    let target_row = row + vertical_move_offset[0];
                    let target_col = col + vertical_move_offset[1];
                    let target_piece = &board.pieces[target_row][target_col];
                    match target_piece {
                        None => {
                            result.push(Position {
                                row: target_row,
                                col: target_col,
                            });
                        }
                        Some(_) => break,
                    }
                }

                let diagonal_move_offsets = [[1, -1], [1, 1]];
                for diagonal_move_offset in diagonal_move_offsets.iter() {
                    // let target_row = row.saturating_add(diagonal_move_offset[0] as usize);
                    // let target_col = col.saturating_add(diagonal_move_offset[1] as usize)
                    let target_row = (row as i32) + diagonal_move_offset[0];
                    let target_col = (col as i32) + diagonal_move_offset[1];
                    let target_row = target_row as usize;
                    let target_col = target_col as usize;
                    let target_piece = &board.pieces[target_row][target_col];
                    match target_piece {
                        None => break,
                        Some(target_piece) => {
                            if color != target_piece.color() {
                                result.push(Position {
                                    row: target_row,
                                    col: target_col,
                                });
                            }
                        }
                    }
                }

                result

                // loop {
                //     let new_col = col + forward_offset;
                //     let new_piece = &board.pieces[row][new_col];

                //     match new_piece {
                //         None => break,
                //         Some(_) => {
                //             v.push(Position { row, col: new_col });
                //             forward_offset += 1;
                //         }
                //     }
                // }
            }
            Piece::Knight(color) => result,
            Piece::Bishop(color) => result,
            Piece::Rook(color) => result,
            Piece::Queen(color) => result,
            Piece::King(color) => result,
        },
    }
}
