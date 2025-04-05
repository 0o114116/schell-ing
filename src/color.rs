use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Color::Black => "B",
            Color::White => "W",
        };

        write!(f, "{}", symbol)
    }
}
