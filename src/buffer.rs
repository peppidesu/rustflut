use std::{collections::HashSet, sync::Mutex};

use rand::seq::SliceRandom;
use rayon::iter::{IntoParallelIterator, ParallelIterator, IntoParallelRefMutIterator};

use crate::{Pixel, Color, Point, WIDTH, HEIGHT, Rect};

#[derive(Debug)]
pub struct PixelBuffer {    
    pixels: Box<[Option<Color>]>
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

    pub fn fill_bounds(&mut self, bounds: Rect, color: Color) {
        let mut bounds = bounds;
        bounds.min.x = bounds.min.x.max(0).min(WIDTH);
        bounds.min.y = bounds.min.y.max(0).min(HEIGHT);
        bounds.max.x = bounds.max.x.max(0).min(WIDTH);
        bounds.max.y = bounds.max.y.max(0).min(HEIGHT);

        for y in bounds.min.y..bounds.max.y {
            let slice = &mut self.pixels[
                bounds.min.x as usize + y as usize * WIDTH as usize
                ..bounds.max.x as usize + y as usize * WIDTH as usize];
            slice.par_iter_mut().for_each(|px| {
                *px = Some(color);
            });
        }
    }    
    pub fn exec_bounds(&mut self, bounds: &Rect, f: fn(&mut Color)) {
        for y in bounds.min.y..bounds.max.y {
            let slice = &mut self.pixels[
                bounds.min.x as usize + y as usize * WIDTH as usize
                ..bounds.max.x as usize + y as usize * WIDTH as usize];
            slice.par_iter_mut().for_each(|px| {
                match px {
                    Some(mut c) => {
                        f(&mut c);
                        *px = Some(c);
                    },
                    None => {}
                }
            });
        }        
    }
    pub fn fill_random(&mut self, bounds: Rect) {
        
        let mut bounds = bounds;
        bounds.min.x = bounds.min.x.max(0).min(WIDTH);
        bounds.min.y = bounds.min.y.max(0).min(HEIGHT);
        bounds.max.x = bounds.max.x.max(0).min(WIDTH);
        bounds.max.y = bounds.max.y.max(0).min(HEIGHT);
        let mut rng = rand::thread_rng();
        let color = Color::random(&mut rng);

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

