use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl Worker {
    pub fn new(id: usize, r: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = r.lock().unwrap().recv().unwrap();
            match msg {
                Message::NewJob(j) => j(),
                Message::Terminate => break,
            }
        });
        let thread = Some(thread);
        Worker {
            id,
            thread,
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, r) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(r));
        let mut threads = Vec::with_capacity(size);
        for i in 0..size {
            threads.push(Worker::new(i + 1, receiver.clone()));
        };
        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.threads {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}