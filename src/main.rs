mod enums;
use enums::{Board, Color, Piece};

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

    println!("{}", board);

    let piece = Piece::Rook(Color::White);
    println!("{}", piece);
    let piece = Piece::Rook(Color::Black);
    println!("{}", piece);
}
