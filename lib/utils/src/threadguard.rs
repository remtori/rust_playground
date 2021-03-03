use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;

const THREAD_RUNNING: u8 = 0;
const THREAD_PANIC: u8 = 1;
const THREAD_FINISH: u8 = 2;

pub struct ThreadGuard(Arc<AtomicU8>);
pub struct ThreadPoker(Arc<AtomicU8>);

pub fn create_thread_guard() -> (ThreadGuard, ThreadPoker) {
    let a = Arc::new(AtomicU8::new(THREAD_RUNNING));

    (ThreadGuard(Arc::clone(&a)), ThreadPoker(Arc::clone(&a)))
}

impl ThreadGuard {
    pub fn im_ok(&self) {}
}

impl ThreadPoker {
    fn status(&self) -> u8 {
        self.0.load(Ordering::Relaxed)
    }

    pub fn is_alive(&self) -> bool {
        self.status() == THREAD_RUNNING
    }

    pub fn is_panicked(&self) -> bool {
        self.status() == THREAD_PANIC
    }

    pub fn is_finish(&self) -> bool {
        self.status() == THREAD_FINISH
    }
}

impl Drop for ThreadGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            self.0.store(THREAD_PANIC, Ordering::Relaxed)
        } else {
            self.0.store(THREAD_FINISH, Ordering::Relaxed)
        }
    }
}
