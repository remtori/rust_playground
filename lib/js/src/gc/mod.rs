use std::{
    any::TypeId,
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

mod block;
mod cell;
mod heap;
mod trace;

pub use block::*;
pub use cell::*;
pub use heap::Heap;
pub use js_derive::GcTrace;
pub use trace::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SweepType {
    Everything,
    Garbage,
}

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GcPointer<T: 'static + GcCell + ?Sized> {
    base: NonNull<Cell>,
    marker: PhantomData<T>,
}

impl<T> GcPointer<T>
where
    T: 'static + GcCell,
{
    pub fn new(cell_ptr: *mut Cell) -> GcPointer<T> {
        GcPointer {
            base: NonNull::new(cell_ptr).unwrap(),
            marker: PhantomData,
        }
    }

    pub unsafe fn cell(&self) -> &mut ManuallyDrop<Cell> {
        &mut *self.base.as_ptr().cast::<_>()
    }

    pub fn gc_mark_alive(&self) {
        unsafe { self.cell().mark() };
    }

    pub fn is<U: GcCell>(&self) -> bool {
        unsafe { self.cell().type_id() == TypeId::of::<U>() }
    }

    pub fn downcast<U: GcCell>(&self) -> Option<GcPointer<U>> {
        if self.is::<U>() {
            Some(GcPointer {
                base: self.base,
                marker: PhantomData,
            })
        } else {
            None
        }
    }
}

impl<T: GcCell> Clone for GcPointer<T> {
    fn clone(&self) -> Self {
        GcPointer {
            base: self.base,
            marker: PhantomData,
        }
    }
}

impl<T> GcPointer<T>
where
    T: 'static + GcCell + Clone,
{
    pub fn clone_inner(&self) -> T {
        self.deref().clone()
    }
}

impl<T> Deref for GcPointer<T>
where
    T: 'static + GcCell,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.base.as_ref().data() }
    }
}

impl<T> DerefMut for GcPointer<T>
where
    T: 'static + GcCell,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.base.as_mut().data() }
    }
}

impl<T> Debug for GcPointer<T>
where
    T: 'static + GcCell + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

unsafe impl<T> Trace for GcPointer<T> where T: 'static + GcCell {}

macro_rules! no_op_trace {
    ($i:ident, $($is:ident),+) => {
       no_op_trace! { $i }
       no_op_trace! { $($is),+ }
    };

    ($i:ident) => {
        unsafe impl Trace for $i {}
    }
}

no_op_trace!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
no_op_trace!(f32, f64, bool, String);

unsafe impl Trace for () {}

unsafe impl<K, V> Trace for HashMap<K, V>
where
    K: Trace,
    V: Trace,
{
    fn trace(&mut self, tracer: &mut Tracer) {
        self.iter_mut().for_each(|(k, v)| {
            // k.trace(tracer);
            v.trace(tracer);
        });
    }
}

unsafe impl<V> Trace for Vec<V>
where
    V: Trace,
{
    fn trace(&mut self, tracer: &mut Tracer) {
        self.iter_mut().for_each(|v| v.trace(tracer))
    }
}
