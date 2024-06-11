
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustflut::*;

fn main() {
    let mut pool = NetWorkerPool::new(8);

    let text_renderer = TextRenderer::new();
    let color = color!(255, 32, 48);
    
    let mut x = 100;
    let mut y = 200;
    let mut dx: i32 = 2;
    let mut dy: i32 = 2;
    
    
    let mut buffer = PixelBuffer::new();
    for _ in 0..100000 {
        clear(&mut buffer);        
        pool.write_px_vec(buffer.get_px_vec())    
    }

}

fn clear(buffer: &mut PixelBuffer) {
    let bounds = Rect::new(
        Point::new(0, 0), 
        Point::new(WIDTH, HEIGHT)
    );
    buffer.fill_bounds(bounds, color!(255, 32, 48));
}
