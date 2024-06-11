use std::fmt::{Display, Formatter, Result};
use rand::*;
#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
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

    pub fn to_cmd(&self) -> Vec<u8> {
        let mut cmd: Vec<u8> = Vec::new();
        
        cmd.push(0xB0);

        cmd.extend(self.x.to_be_bytes());
        cmd.extend(self.y.to_be_bytes());

        cmd.push(self.color.r);
        cmd.push(self.color.g);
        cmd.push(self.color.b);

        cmd
    }
}