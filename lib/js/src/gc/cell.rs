use std::{any::TypeId, mem::size_of, ptr::DynMetadata};

use super::Trace;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellStatus {
    Marked,
    Unmarked,
}

pub trait GcCell: Trace + std::any::Any {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Cell {
    pub(crate) next: *mut Cell,
    status: CellStatus,
    type_id: TypeId,
    vtable: Option<DynMetadata<dyn GcCell>>,
}

impl Cell {
    pub(crate) unsafe fn init_free(ptr: *mut Cell, next: *mut Cell) {
        ptr.write(Cell {
            next,
            status: CellStatus::Unmarked,
            type_id: TypeId::of::<()>(),
            vtable: None,
        });
    }

    pub(crate) unsafe fn placement_new<T>(ptr: *mut Cell, next: *mut Cell, data: T)
    where
        T: 'static + GcCell,
    {
        let vtable = std::ptr::metadata(&data as &dyn GcCell);

        ptr.write(Cell {
            next,
            status: CellStatus::Unmarked,
            type_id: TypeId::of::<T>(),
            vtable: Some(vtable),
        });

        ((ptr as *mut u8).add(size_of::<Self>()) as *mut T).write(data);
    }

    pub fn data<T>(&self) -> *mut T {
        self.data_ptr() as *mut T
    }

    pub fn get_dyn(&self) -> &mut dyn GcCell {
        unsafe {
            &mut *std::ptr::from_raw_parts_mut::<dyn GcCell>(
                self.data_ptr() as *mut (),
                self.vtable
                    .clone()
                    .expect("Cell::get_dyn() called on freed/empty cell"),
            )
        }
    }

    fn data_ptr(&self) -> usize {
        unsafe { (self as *const Self as *mut u8).add(size_of::<Self>()) as usize }
    }

    pub fn mark(&mut self) {
        self.status = CellStatus::Marked;
    }

    pub fn unmark(&mut self) {
        self.status = CellStatus::Unmarked;
    }

    pub fn is_marked(&self) -> bool {
        self.status == CellStatus::Marked
    }

    pub fn is_free(&self) -> bool {
        self.vtable.is_none()
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}
