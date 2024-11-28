use colored::*;
use core::fmt;

#[derive(Debug)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[derive(Debug)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (letter, color) = match self {
            Piece::Pawn(color) => ('P', color),
            Piece::Knight(color) => ('N', color),
            Piece::Bishop(color) => ('B', color),
            Piece::Rook(color) => ('R', color),
            Piece::Queen(color) => ('Q', color),
            Piece::King(color) => ('K', color),
        };

        write!(
            f,
            "{}",
            format!("{}", letter).color(match color {
                Color::White => "white",
                Color::Black => "black",
            })
        )

        // println!("{}", "Too small".red());

        // match color {
        //     Color::White => {
        //         write!(f, "{}", format!("{}", letter.to_string().white()))
        //     }
        //     Color::Black => {
        //         write!(f, "{}", format!("{}", letter.to_string().black()))
        //     }
        // }

        // write!(
        //     f,
        //     "{}",
        //     format!("{}", letter).to_string().color(match color {
        //         Color::White => "green",
        //         Color::Black => "red",
        //     })
        // )
    }
}
