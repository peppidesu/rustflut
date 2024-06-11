use std::fmt::{Display, Formatter, Result};

use rand::{Rng, rngs::ThreadRng};

use crate::{point, Point};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[inline]
fn i2f(i: u8) -> f32 {
    i as f32 / 255.0
}
#[inline]
fn f2i(f: f32) -> u8 {
    (f * 255.0) as u8
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_str(s: &str) -> Color {
        // format rrggbb
        let r = u8::from_str_radix(&s[0..2], 16).unwrap();
        let g = u8::from_str_radix(&s[2..4], 16).unwrap();
        let b = u8::from_str_radix(&s[4..6], 16).unwrap();
        
        Color { r, g, b, a: 255 }
    }

    pub fn random(rng: &mut ThreadRng) -> Color {
        
        Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
            a: 255
        }
    }


    pub fn add(&self, other: &Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
            a: self.a.saturating_add(other.a)
        }
    }

    pub fn sub(&self, other: &Color) -> Color {
        Color {
            r: self.r.saturating_sub(other.r),
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b),
            a: self.a.saturating_sub(other.a)
        }
    }

    pub fn mul(&self, other: &Color) -> Color {
        Color {
            r: f2i(i2f(self.r) * i2f(other.r)),
            g: f2i(i2f(self.g) * i2f(other.g)),
            b: f2i(i2f(self.b) * i2f(other.b)),
            a: f2i(i2f(self.a) * i2f(other.a))
        }
    }
    pub fn mix(&self, other: Color) -> Color {
        let a = 1. - (1. - i2f(self.a)) * (1. - i2f(other.a));
        let fac1 = i2f(self.a) / a;
        let fac2 = (1. - i2f(other.a)) / a;
        let r = i2f(other.r) * fac1 + i2f(self.r) * fac2;
        let g = i2f(other.g) * fac1 + i2f(self.g) * fac2;
        let b = i2f(other.b) * fac1 + i2f(self.b) * fac2;
        Color {
            r: f2i(r),
            g: f2i(g),
            b: f2i(b),
            a: f2i(a)
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



#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub pos: Point,
    pub color: Color
}


impl Pixel {
    pub fn new(pos: Point, color: Color) -> Pixel {
        Pixel {
            pos,
            color,
        }
    }

    pub fn from_str(s: &str) -> Pixel {
        let mut iter = s.split_whitespace();
        
        iter.next();
        
        let x = iter.next().unwrap().parse::<u16>().unwrap();
        let y = iter.next().unwrap().parse::<u16>().unwrap();
        let color = Color::from_str(iter.next().unwrap());
        
        Pixel {
            pos: point!(x, y),
            color
        }
    }

    pub fn to_cmd(&self) -> Vec<u8> {
        let mut cmd: Vec<u8> = Vec::with_capacity(8);

        cmd.push(0xB0);
        cmd.extend(self.pos.x.to_be_bytes());
        cmd.extend(self.pos.y.to_be_bytes());
        cmd.push(self.color.r);
        cmd.push(self.color.g);
        cmd.push(self.color.b);
        cmd
    }
}

#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color::rgb($r, $g, $b)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::rgba($r, $g, $b, $a)
    };
    ($hex:expr) => {
        Color::from_str($hex)
    };
    ($c:expr, $a:expr) => {
        Color::rgba($c.r, $c.g, $c.b, $a)
    };
}