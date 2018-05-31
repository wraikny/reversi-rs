use std::fmt;

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn rev(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }

    pub fn sym(&self) -> &str {
        match self {
            Color::Black => "*",
            Color::White => "o",
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        use color::Color::{Black, White};
        
        match self {
            White => { match other {
                White => true,
                Black => false,
            }},
            Black => { match other {
                White => false,
                Black => true,
            }},
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", match self {
            Color::Black => "Black",
            Color::White => "White",
        }, 
        self.sym()
        )
    }
}