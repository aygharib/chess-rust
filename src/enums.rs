use colored::*;
use core::fmt;

// Wrap the 2-d array in a struct so I can implement display trait for it
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.pieces {
            for piece in row {
                let output_string = match piece {
                    Some(piece) => piece.to_string(),
                    None => ".".to_string(),
                };
                write!(f, "{} ", output_string)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn color(&self) -> &Color {
        match self {
            Piece::Pawn(color) => color,
            Piece::Knight(color) => color,
            Piece::Bishop(color) => color,
            Piece::Rook(color) => color,
            Piece::Queen(color) => color,
            Piece::King(color) => color,
        }
    }
    // fn has_same_color(&self, other: &Piece) -> bool {
    //     self.color()
    // }
}

#[derive(Debug, PartialEq)]
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
                Color::White => "green",
                Color::Black => "red",
            })
        )
    }
}

#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
