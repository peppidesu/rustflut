
use rustflut::*;
use rand::*;



fn main() {
    
    let mut pool = NetWorkerPool::new();

    let text_renderer = TextRenderer::new();
    let color = Color::rgb(255, 0, 0);
    for _ in 0..100000 {        
       
        let text = "rustflut";
        let text_px_vec = text_renderer.render(text, color.clone());
        pool.write_px_vec(text_px_vec);
    }
    
    
}



