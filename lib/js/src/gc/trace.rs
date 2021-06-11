use crate::GcCell;

pub struct Tracer {}

impl Tracer {
    pub fn visit<T>(&self, obj: &crate::Gc<T>)
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
