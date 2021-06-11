#![feature(ptr_metadata)]

use std::{
    any::TypeId,
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
pub use trace::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SweepType {
    Everything,
    Garbage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GcPointer<T: 'static + GcCell> {
    base: NonNull<T>,
}

impl<T> GcPointer<T>
where
    T: 'static + GcCell,
{
    pub fn new(cell: &Cell) -> GcPointer<T> {
        GcPointer {
            base: NonNull::new(cell.data() as *mut T).unwrap(),
        }
    }

    pub unsafe fn cell(&self) -> &mut ManuallyDrop<Cell> {
        let cell_ptr = (self.base.as_ptr() as *mut u8).sub(std::mem::size_of::<Cell>());
        &mut *(cell_ptr as *mut Cell).cast::<_>()
    }

    pub fn gc_mark_alive(&self) {
        unsafe {
            self.cell().mark();
        }
    }

    pub fn is<U: GcCell>(&self) -> bool {
        unsafe { self.cell().type_id() == TypeId::of::<U>() }
    }

    pub fn downcast<U: GcCell>(&self) -> Option<GcPointer<U>> {
        if self.is::<U>() {
            Some(GcPointer {
                base: self.base.cast(),
            })
        } else {
            None
        }
    }
}

impl<T> Deref for GcPointer<T>
where
    T: 'static + GcCell,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.base.as_ref() }
    }
}

impl<T> DerefMut for GcPointer<T>
where
    T: 'static + GcCell,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.base.as_mut() }
    }
}
