use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b,
            a: 255
        }
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r,
            g,
            b,
            a
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.a == 255 {
            write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        }
        else {
            write!(f, "{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
}


#[derive(Clone)]
pub struct Pixel {
    x: u16,
    y: u16,
    color: Color
}

impl Pixel {
    pub fn new(x: u16, y: u16, color: Color) -> Pixel {
        Pixel {
            x,
            y,
            color
        }
    }

    pub fn to_string(&self) -> String {
        format!("PX {} {} {}", self.x, self.y, self.color)
    }
}