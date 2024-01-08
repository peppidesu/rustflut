use std::fmt::{Display, Formatter, Result};

#[derive(Clone,Copy)]
pub struct Pos {
    pub x: u16,
    pub y: u16
}

impl Pos {
    pub fn new(x: u16, y: u16) -> Pos {
        Pos {
            x,
            y
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

pub struct Bounds {
    pub min: Pos,
    pub max: Pos
}

impl Bounds {
    pub fn new(min: Pos, max: Pos) -> Bounds {
        Bounds {
            min,
            max
        }
    }
}

impl Display for Bounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.min, self.max)
    }
}