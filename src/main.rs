mod enums;
use enums::{Color, Piece};

fn main() {
    let piece = Piece::Rook(Color::White);
    println!("{:#?}", piece);
}
