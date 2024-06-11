use std::{net::TcpStream, sync::Mutex};
use std::io::prelude::*;


use crate::{pixel::*, Rect, Point, PixelBuffer};

const HOST: &str = "pixelflut.peppidesu.dev:55282";

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
        let msg = px.to_cmd();
        self.stream.write(&msg).unwrap();
    }

    

    pub fn get_px_vec(&mut self, points: Vec<Point>) -> Vec<Pixel> {
        let mut msg = String::new();
        for pos in points.iter() {            
            msg.push_str(&format!("PX {} {}\n", pos.x, pos.y));
        }
        
        self.stream.write(msg.as_bytes()).unwrap();
        
        let mut msg = String::new();
        
        let mut buf = [0; 2048];
        let mut nlcount = 1;
        
        loop {            
            let len = self.stream.read(&mut buf).unwrap();            
            let s = String::from_utf8_lossy(&buf[..len]);
            
            msg.push_str(&s);            
            nlcount += s.matches("\n").count();
            
            if nlcount >= points.len() {                
            
                
                break;
            }
            
        }
        
        msg.split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| Pixel::from_str(s))
            .collect::<Vec<Pixel>>()
    }

    pub fn write_px_vec(&mut self, px_vec: Vec<Pixel>) {
        // use write_all
        let msg = px_vec.into_iter()
            .map(|px| px.to_cmd())
            .flatten()
            .collect::<Vec<u8>>();            
        self.stream.write_all(&msg).unwrap();
    }

    pub fn get_px_bounds(&mut self, bounds: &Rect) -> Vec<Pixel> {
        
        let mut point_vec = Vec::new();
        
        (bounds.min.y..bounds.max.y).into_iter().for_each(|y| {
            (bounds.min.x..bounds.max.x).into_iter().for_each(|x| {
                point_vec.push(Point::new(x, y));
            });
        });

        self.get_px_vec(point_vec)
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


    pub fn get_px_bounds(&mut self, bounds: &Rect, buffer: &mut PixelBuffer) {
        let buffer = Mutex::new(buffer);
        
        
        let chunk_height = (bounds.max.y - bounds.min.y) / self.workers.len() as u16;
        let chunks = (bounds.min.y..bounds.max.y)
            .step_by(chunk_height as usize)
            .map(|y| {
                
                let min = Point::new(bounds.min.x, y);
                let max = Point::new(bounds.max.x, y + chunk_height);
                Rect::new(min, max)
            });
        
        self.pool.scope(|s| {
            let buffer = &buffer;
            for (worker, chunk) in self.workers.iter_mut().zip(chunks) {
                s.spawn(move |_| {
                    
                    let mut thread_px_vec = worker.get_px_bounds(&chunk);
                     
                    let mut buffer = buffer.lock().unwrap();
                    for px in thread_px_vec.drain(..) {
                        buffer.set(px);
                        
                    }
                });
            }
        });
        

        
    }
}