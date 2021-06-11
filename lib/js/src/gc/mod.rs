#![feature(ptr_metadata)]

use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
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

#[derive(Debug, Clone, Copy)]
pub struct Gc<T: 'static + GcCell> {
    ptr: *mut T,
}

impl<T> Gc<T>
where
    T: 'static + GcCell,
{
    pub fn new(cell: &Cell) -> Gc<T> {
        Gc {
            ptr: cell.data() as *mut T,
        }
    }

    pub unsafe fn cell(&self) -> &mut ManuallyDrop<Cell> {
        &mut *((self.ptr as *mut u8).sub(std::mem::size_of::<Cell>()) as *mut Cell).cast::<_>()
    }

    pub fn mark(&self) {
        unsafe {
            self.cell().mark();
        }
    }
}

impl<T> Deref for Gc<T>
where
    T: 'static + GcCell,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T> DerefMut for Gc<T>
where
    T: 'static + GcCell,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}
