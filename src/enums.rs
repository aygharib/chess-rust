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
                // match piece {
                //     Some(piece) => piece,
                //     None => None(Piece),
                // }
                // write!(f, "{}", piece)?;
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

        // println!("{}", "Too small".red());
        // println!("{}", "R".white().on_truecolor(128, 128, 128));
        // println!("{}", "R".black().on_truecolor(192, 192, 192));
        // let my_string = String::new("nice");
        // my_string.redgc

        write!(
            f,
            "{}",
            format!("{}", letter).color(match color {
                Color::White => "green",
                Color::Black => "red",
            })
        )

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

// impl fmt::Display for Option<Piece> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Some(piece) => write!(f, "{}", piece),
//             None => write!(f, "."),
//         }
//     }
// }
