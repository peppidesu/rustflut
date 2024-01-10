use std::{net::TcpStream, sync::Mutex};
use std::io::prelude::*;


use crate::{pixel::*, Bounds, Point};

const HOST: &str = "pixelflut.uwu.industries:1234";

pub struct NetWorker {
    stream: TcpStream
}

impl NetWorker {
    pub fn new() -> NetWorker {
        let stream = TcpStream::connect(HOST).unwrap();
        NetWorker {
            stream
        }
    }

    pub fn write_px(&mut self, px: &Pixel) {
        let msg = px.to_string();
        self.stream.write(msg.as_bytes()).unwrap();
    }

    pub fn get_px(&mut self, pos: Point) -> Pixel {           
        let msg = format!("PX {} {}", pos.x, pos.y);
        self.stream.write(msg.as_bytes()).unwrap();

        let mut buf = [0; 24];
        
        
        let len = self.stream.read(&mut buf).unwrap();   
        
        Pixel::from_str(&String::from_utf8_lossy(&buf[..len]))       
    }

    pub fn get_px_vec(&mut self, points: Vec<Point>) -> Vec<Pixel> {
        let mut msg = String::new();
        for pos in points.iter() {            
            msg.push_str(&format!("PX {} {}\n", pos.x, pos.y));
        }
        
        self.stream.write(msg.as_bytes()).unwrap();
        
        let mut msg = String::new();
        
        let mut buf = [0; 128];
        loop {
            let len = self.stream.read(&mut buf).unwrap();
                     
            
            msg.push_str(&String::from_utf8_lossy(&buf[..len]));            
            if len < 128 {                
                break;
            }
        }
        println!("{}", msg);
        msg.split("\n")
            .map(|s| Pixel::from_str(s))
            .collect::<Vec<Pixel>>()
    }

    pub fn write_px_vec(&mut self, px_vec: Vec<Pixel>) {
        // use write_all
        let msg = px_vec.into_iter()
            .map(|px| px.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let bytes = msg.as_bytes();
        self.stream.write_all(bytes).unwrap();
    }

    pub fn get_px_bounds(&mut self, bounds: &Bounds) -> Vec<Pixel> {
        
        let mut px_vec = Vec::new();
        for y in bounds.min.y..bounds.max.y {
            for x in bounds.min.x..bounds.max.x {
                
                let pos = Point::new(x, y);
                let px = self.get_px(pos);
                px_vec.push(px);
            }
        }
        px_vec
    }
}

pub struct NetWorkerPool {
    workers: Vec<NetWorker>,
    pool: rayon::ThreadPool
}

impl NetWorkerPool {
    pub fn new(num_threads: usize) -> NetWorkerPool {
        let mut workers = Vec::new();
        for _ in 0..num_threads {
            let worker = NetWorker::new();
            workers.push(worker);
        }

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap();        
        
        NetWorkerPool { workers, pool }
    }

    pub fn write_px_vec(&mut self, px_vec: Vec<Pixel>) {

        let chunks = px_vec.chunks(px_vec.len() / self.workers.len()).collect::<Vec<_>>();

        self.pool.scope(|s| {
            for (worker, chunk) in self.workers.iter_mut().zip(chunks) {
                s.spawn(move |_| {
                    worker.write_px_vec(chunk.to_vec());
                    
                });
            }
        });
    }


    pub fn get_px_region(&mut self, bounds: Bounds) -> Vec<Pixel> {
        let px_vec = Mutex::new(Vec::new());
        
        
        let chunk_height = (bounds.max.y - bounds.min.y) / self.workers.len() as u16;
        let chunks = (bounds.min.y..bounds.max.y)
            .step_by(chunk_height as usize)
            .map(|y| {
                
                let min = Point::new(bounds.min.x, y);
                let max = Point::new(bounds.max.x, y + chunk_height);
                Bounds::new(min, max)
            });
        
        self.pool.scope(|s| {
            let px_vec = &px_vec;
            for (worker, chunk) in self.workers.iter_mut().zip(chunks) {
                s.spawn(move |_| {
                     
                    let mut thread_px_vec = worker.get_px_bounds(&chunk);
                     
                    px_vec.lock().unwrap().append(&mut thread_px_vec);
                });
            }
        });


        px_vec.into_inner().unwrap()
    }
}