use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

pub struct ThreatPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Newjob(Job),
    Terminate,
}
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
        self.sender.send(Message::Newjob(job)).unwrap();
    }
}
impl Drop for ThreatPool {
    fn drop(&mut self) {
        println!("Sending terminat Message");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");
        for worker in &mut self.workers {
            println!("shutting down {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Newjob(job) => {
                    println!("Worker {} is on a job", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was terminated ", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
