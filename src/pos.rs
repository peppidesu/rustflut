use std::fmt::{Display, Formatter, Result};

#[derive(Clone,Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Bounds {
    pub min: Point,
    pub max: Point
}

impl Bounds {
    pub fn new(min: Point, max: Point) -> Bounds {
        Bounds { min, max }
    }
}

impl Display for Bounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.min, self.max)
    }
}