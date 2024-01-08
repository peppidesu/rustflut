
use std::{thread, time::Duration};

use rustflut::*;
use rand::*;

const WIDTH: u16 = 1280;
const HEIGHT: u16 = 720;

fn main() {
    
    let mut pool = NetWorkerPool::new();

    let text_renderer = TextRenderer::new();
    let color = Color::rgb(255, 32, 48);
    
    let mut y = 200;
    let mut dy: i32 = 1;
    for _ in 0..100000 {        
        if (y > 500) || (y < 200) {
            dy = -dy;
        }
        y += dy;
        let text = "rust rules!";
        let text_px_vec = text_renderer.render_bg(text, color.clone(), Pos::new(100, y as u16));
        pool.write_px_vec(text_px_vec);

        let text = "repo: peppidesu/rustflut";
        let text_px_vec = text_renderer.render_bg(text, color.clone(), Pos::new(100, y as u16 + 50));
        pool.write_px_vec(text_px_vec);
        
    }
    
    
}

fn clear(pool: &mut NetWorkerPool) {
    let mut px_vec = Vec::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let px = Pixel::new(Pos::new(x, y), Color::rgb(0, 0, 0));
            px_vec.push(px);
        }
    }
    pool.write_px_vec(px_vec);
    
}

