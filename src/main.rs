
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustflut::*;

fn main() {
    
    
    let mut pool = NetWorkerPool::new(8);

    let text_renderer = TextRenderer::new();
    let color = Color::rgb(255, 32, 48);
    
    let mut x = 100;
    let mut y = 200;
    let mut dx: i32 = 1;
    let mut dy: i32 = 1;
    
    
    let mut buffer = PixelBuffer::new();
    for _ in 0..100000 {     
        buffer.clear();
        
        if (x > 1040) || (x < 5) {
            dx = -dx;
        }
        if (y > 620) || (y < 5) {
            dy = -dy;
        }
        x += dx;
        y += dy;        
        
        text_renderer.render_bg("Dvd logo", color.clone(), Point::new(x as u16, y as u16), &mut buffer);        
        text_renderer.render_bg("rustflut", color.clone(), Point::new(x as u16, y as u16 + 50), &mut buffer);

        pool.write_px_vec(buffer.get_px_vec());             
    }

}

fn clear() -> Vec<Pixel> {      
    (0..HEIGHT).into_par_iter().map(|y| {        
        let mut rng = rand::thread_rng();
        let mut thread_px_vec = Vec::new();
        let color = Color::random(& mut rng);
        for x in 0..WIDTH {
            let px = Pixel::new(Point::new(x, y), color);
            thread_px_vec.push(px);
        }
        thread_px_vec
    }).flatten().collect()      
}

fn sub(pool: &mut NetWorkerPool) {
    let mut px_vec = pool.get_px_region(
        Bounds::new(
            Point::new(0, 0), 
            Point::new(WIDTH, HEIGHT)
        )
    );    
    px_vec = px_vec.iter().map(|px| {
        let color = Color::rgb(255, 0, 0);
        let mut px = *px;
        px.color = color.sub(&px.color);
        px        
    }).collect();
    
    pool.write_px_vec(px_vec);
}

