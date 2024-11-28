mod enums;
use enums::{Color, Piece};

fn main() {
    // let board = [
    //     [
    //         Piece::Rook(Color::White),
    //         Piece::Knight(Color::White),
    //         Piece::Bishop(Color::White),
    //         Piece::Queen(Color::White),
    //         Piece::King(Color::White),
    //         Piece::Bishop(Color::White),
    //         Piece::Knight(Color::White),
    //         Piece::Rook(Color::White),
    //     ],
    //     [
    //         Piece::Pawn(Color::White),
    //         Piece::Pawn(Color::White),
    //         Piece::Pawn(Color::White),
    //         Piece::Pawn(Color::White),
    //         Piece::Pawn(Color::White),
    //         Piece::Pawn(Color::White),
    //         Piece::Pawn(Color::White),
    //         Piece::Pawn(Color::White),
    //     ],
    //     [
    //         Piece::Pawn(Color::Black),
    //         Piece::Pawn(Color::Black),
    //         Piece::Pawn(Color::Black),
    //         Piece::Pawn(Color::Black),
    //         Piece::Pawn(Color::Black),
    //         Piece::Pawn(Color::Black),
    //         Piece::Pawn(Color::Black),
    //         Piece::Pawn(Color::Black),
    //     ],
    //     [
    //         Piece::Rook(Color::Black),
    //         Piece::Knight(Color::Black),
    //         Piece::Bishop(Color::Black),
    //         Piece::Queen(Color::Black),
    //         Piece::King(Color::Black),
    //         Piece::Bishop(Color::Black),
    //         Piece::Knight(Color::Black),
    //         Piece::Rook(Color::Black),
    //     ],
    // ];

    let board = [
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
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
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
    ];

    // println!("{:#?}", board);
    println!("{:?}", board);

    let piece = Piece::Rook(Color::White);
    println!("{}", piece);
}
