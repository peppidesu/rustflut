
use std::{thread, time::Duration};

use rustflut::*;
use rand::*;

const WIDTH: u16 = 1280;
const HEIGHT: u16 = 720;

fn main() {
    
    let mut pool = NetWorkerPool::new();

    let text_renderer = TextRenderer::new();
    let color = Color::rgb(255, 0, 0);
    clear(&mut pool);
    thread::sleep(Duration::from_millis(100));
    for _ in 0..100000 {        
        
        let text = "repo: peppidesu/rustflut";
        let text_px_vec = text_renderer.render(text, color.clone());
        pool.write_px_vec(text_px_vec);
        thread::sleep(Duration::from_millis(10));
    }
    
    
}

fn clear(pool: &mut NetWorkerPool) {
    let mut px_vec = Vec::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let px = Pixel::new(x, y, Color::rgb(0, 0, 0));
            px_vec.push(px);
        }
    }
    pool.write_px_vec(px_vec);
    
}

