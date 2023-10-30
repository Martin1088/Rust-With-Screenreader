use std::{
    process::id,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

pub struct ThreatPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreatPool {
    pub fn new(size: usize) -> ThreatPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreatPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} is one a job", id);
            job()
        });
        Worker { id, thread }
    }
}
