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
pub struct Rect {
    pub min: Point,
    pub max: Point
}

impl Rect {
    pub fn new(min: Point, max: Point) -> Rect {
        Rect { min, max }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.min, self.max)
    }
}

#[macro_export]
macro_rules! rect_xywh {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new(Point::new($x, $y), Point::new($x + $w, $y + $h))
    };
}

#[macro_export]
macro_rules! rect_p1p2 {
    ($x1:expr, $y1:expr, $x2:expr, $y2:expr) => {
        Rect::new(Point::new($x1, $y1), Point::new($x2, $y2))
    };
}

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::new($x, $y)
    };
}