use std::sync::{Arc, Mutex};

use crate::gc::{Heap, SweepType};

pub struct Runtime {
    heap: Heap,
}

impl Runtime {
    pub fn new() -> Arc<Mutex<Runtime>> {
        Arc::new(Mutex::new(Runtime {
            heap: Default::default(),
        }))
    }

    pub fn heap(&mut self) -> &mut Heap {
        &mut self.heap
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.heap.collect_garbage(SweepType::Everything);
    }
}
