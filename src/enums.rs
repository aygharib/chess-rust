#[derive(Debug)]
pub enum Piece {
    Rook(Color),
    Bishop(Color),
    King(Color),
}

#[derive(Debug)]
pub enum Color {
    White,
    Black,
}
