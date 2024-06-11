pub const WIDTH: u16 = 800;
pub const HEIGHT: u16 = 600;
mod pixel;
mod worker;
mod text;
#[macro_use]
mod pos;
mod buffer;



pub use pixel::*;
pub use worker::*;
pub use text::*;
pub use pos::*;
pub use buffer::*;

use std::collections::HashMap;
pub fn vec_to_posmap(px_vec: Vec<Pixel>) -> HashMap<Point, Pixel> {
    px_vec.into_iter()
        .map(|px| (px.pos, px))
        .collect()
}