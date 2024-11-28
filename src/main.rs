/*
Design

2-dimensional array to store the game board state

important data
- which pieces exist
- which piece color can move
- which side is in check

We need a way to know who's turn it is to play

translate_from_chess_coordinates_to_array_coordinates
-> converts "a5" to "04"

array<array<Tile, 8>, 8> chess_board{};

enum Tile {
    Empty,
    Piece,
}

struct Piece {
    Type,
    Color
}

enum Type {
    Rook,
    Bishop,
    King,
}

enum Color {
    White,
    Black,
}

enum PlaySide {
    White,
    Black,
}

enum PieceType {
    Rook,
    Bishop,
    King,
}

enum PieceColor {
    White,
    Black,
}

Given a game state 2-d array, how can we easily determine if a King is in check?
- precondition: we know that if white can move a piece, then 100% black is not currently in check
- when we move a piece to (destX, destY), if that piece's "threatened cells" contains the enemy king, then we know the enemy is now in check

Given a game state 2-d array where the player to play is in check, how can we determine that a move will remove the check state?
- precondition: we know that if black is in check, then 100% black needs to make a move that results in him no longer being in check
-
*/
// struct Piece {
//     kind: Type,
//     color: Color,
// }

// enum Type {
//     Rook,
//     Bishop,
//     King,
// }

// enum Color {
//     White,
//     Black,
// }

// I don't think I need a tile with two variants, Empty and Piece?
// Isn't that what the Some() and None() Result type is for? Seems redundant
// enum Tile {
//     Empty,

// }

enum Piece {
    Root(Color),
    Bishop(Color),
    King(Color),
}

enum Color {
    White,
    Black,
}

// struct Piece {
//     type: String,
//     // type: Type,
//     // color: Color,
// }

fn main() {
    println!("Hello, world!");
}
