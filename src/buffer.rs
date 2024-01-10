use std::{collections::HashSet, sync::Mutex};

use rayon::iter::{IntoParallelIterator, ParallelIterator, IntoParallelRefMutIterator};

use crate::{Pixel, Color, Point, WIDTH, HEIGHT, Bounds};

pub struct PixelBuffer {    
    pixels: Box<[Option<Color>]>,
}


impl PixelBuffer {
    pub fn new() -> PixelBuffer {                
        PixelBuffer {        
            pixels: vec![None; WIDTH as usize * HEIGHT as usize].into_boxed_slice(),
        }
    }

    pub fn get(&self, pos: Point) -> Option<Pixel> {
        if pos.x >= WIDTH || pos.y >= HEIGHT {
            return None;
        }
        match self.pixels[pos.x as usize + pos.y as usize * WIDTH as usize] {
            Some(color) => Some(Pixel::new(pos, color)),
            None => None,
        }
    }

    pub fn set(&mut self, px: Pixel) {
        if px.pos.x >= WIDTH || px.pos.y >= HEIGHT {
            return;
        }
        if px.color.a == 255 {
            self.pixels[px.pos.x as usize + px.pos.y as usize * WIDTH as usize] = Some(px.color);
            return;
        }
        // alpha blend
        let old_color = self.pixels[px.pos.x as usize + px.pos.y as usize * WIDTH as usize];
        match old_color {
            Some(old_color) => {
                let new_color = old_color.mix(px.color);
                self.pixels[px.pos.x as usize + px.pos.y as usize * WIDTH as usize] = Some(new_color);
            },
            None => {
                self.pixels[px.pos.x as usize + px.pos.y as usize * WIDTH as usize] = Some(px.color);
            }
        }
    }

    pub fn get_px_vec(&self) -> Vec<Pixel> {
        (0..WIDTH).into_par_iter().map(|x| {
            (0..HEIGHT).into_par_iter().filter_map(|y| {
                let pos = Point::new(x, y);
                self.get(pos)
            }).collect::<Vec<Pixel>>()
        }).flatten().collect()
    }

    pub fn fill_bounds(&mut self, bounds: Bounds, color: Color) {
        for y in bounds.min.y..bounds.max.y {
            let slice = &mut self.pixels[
                bounds.min.x as usize + y as usize * WIDTH as usize
                ..bounds.max.x as usize + y as usize * WIDTH as usize];
            slice.par_iter_mut().for_each(|px| {
                *px = Some(color);
            });
        }
    }    

    pub fn clear(&mut self) {
        // in place
        self.pixels.par_iter_mut().for_each(|px| {
            *px = None;
        });
    }
}

