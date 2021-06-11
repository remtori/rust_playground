use super::{GcCell, GcPointer};

#[derive(Debug)]
pub struct Tracer {}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {}
    }

    pub fn visit<T: GcCell>(&mut self, obj: &mut GcPointer<T>) {
        unsafe {
            obj.cell().mark();
        }
    }
}

pub unsafe trait Trace {
    fn trace(&mut self, tracer: &mut Tracer) {
        let _ = tracer;
    }
}
