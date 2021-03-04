use crate::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use std::*;

pub struct ThreadPool {
    uid: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
}

type Job = Box<dyn FnOnce() + panic::UnwindSafe + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut uid = 0usize;

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(uid, Arc::clone(&receiver)));
            uid += 1;
        }

        ThreadPool {
            uid,
            workers,
            sender,
            receiver,
        }
    }

    pub fn respawn_worker_if_needed(&mut self) {
        let size = self.workers.len();

        for worker in self.workers.iter_mut() {
            if !worker.is_alive() {
                info!("Worker {} died!", worker.id());
                if let Some(thread) = worker.thread.take() {
                    let _ = thread.join();
                }
            }
        }

        self.workers.retain(|worker| worker.is_alive());

        // The amount of worker that we need to respawn
        let size = size - self.workers.len();

        for _ in 0..size {
            self.workers
                .push(Worker::new(self.uid, Arc::clone(&self.receiver)));
            self.uid += 1;
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + panic::UnwindSafe + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    poker: ThreadPoker,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let (guard, poker) = create_thread_guard();

        let thread = thread::Builder::new()
            .name(format!("Worker {}", id))
            .spawn(move || loop {
                guard.im_ok();
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        // We do not really care if the job panicked, its still consider it done
                        let _ = std::panic::catch_unwind(job);
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            })
            .unwrap();

        Worker {
            id,
            thread: Some(thread),
            poker,
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn is_alive(&self) -> bool {
        self.poker.is_alive()
    }
}
