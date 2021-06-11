use super::{GcCell, GcPointer};

#[derive(Debug)]
pub struct Tracer {}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {}
    }

    pub fn visit<T>(&self, obj: &GcPointer<T>)
    where
        T: 'static + GcCell,
    {
        unsafe {
            obj.cell().mark();
        }
    }
}

pub trait Trace {
    fn trace(&mut self, tracer: &mut Tracer);
}
