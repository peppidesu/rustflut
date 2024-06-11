
use rustflut::*;
use rand::*;



fn main() {
    
    let mut pool = NetWorkerPool::new();


    for _ in 0..300 {
        let mut px_vec = Vec::new();
        let r = rand::thread_rng().gen_range(0..255);
        let g = rand::thread_rng().gen_range(0..255);
        let b = rand::thread_rng().gen_range(0..255);
        let color = Color::new(r, g, b);

        for x in 0..800 {
            for y in 0..600 {
                let px = Pixel::new(x, y, color.clone());
                px_vec.push(px);
            }
        }
        pool.write_px_vec(px_vec);
    }
    
    
}



