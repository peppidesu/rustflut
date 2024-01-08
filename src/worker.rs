use std::net::TcpStream;
use std::io::prelude::*;
use crate::pixel::*;

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

    pub fn write_px_vec(&mut self, px_vec: Vec<Pixel>) {
        // use write_all
        let msg = px_vec.iter()
            .map(|px| px.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let bytes = msg.as_bytes();
        self.stream.write_all(bytes).unwrap();
    }
}

pub struct NetWorkerPool {
    workers: Vec<NetWorker>,
    pool: rayon::ThreadPool
}

impl NetWorkerPool {
    pub fn new() -> NetWorkerPool {
        let mut workers = Vec::new();

        for _ in 0..12 {
            let worker = NetWorker::new();
            workers.push(worker);
        }
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(12)
            .build()
            .unwrap();
        
        
        NetWorkerPool {
            workers,
            pool
        }
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
}