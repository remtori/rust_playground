use super::{Cell, GcCell, GcPointer, SweepType};

#[repr(C)]
#[derive(Debug)]
pub struct HeapBlock {
    cell_size: usize,
    free_cell: *mut Cell,
    used_cell: *mut Cell,
    ptr: *mut u8,
}

pub const BLOCK_SIZE: usize = 4 * 1024;

impl HeapBlock {
    pub fn new(cell_size: usize) -> HeapBlock {
        debug_assert!(cell_size.is_power_of_two());
        debug_assert!(BLOCK_SIZE % cell_size == 0);

        let layout = std::alloc::Layout::from_size_align(BLOCK_SIZE, cell_size).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) } as *mut Cell;

        let mut block = HeapBlock {
            cell_size,
            free_cell: std::ptr::null_mut(),
            used_cell: std::ptr::null_mut(),
            ptr: ptr as *mut u8,
        };

        block.recycle(cell_size);

        block
    }

    pub fn allocate<T>(&mut self, data: T) -> GcPointer<T>
    where
        T: Sized + 'static + GcCell,
    {
        debug_assert!(self.has_empty_slot());
        debug_assert!(self.cell_size >= std::mem::size_of::<T>() + std::mem::size_of::<Cell>());

        let cell_ptr = self.free_cell;
        self.free_cell = unsafe { (*self.free_cell).next };

        unsafe { Cell::placement_new(cell_ptr, self.used_cell, data) };
        self.used_cell = cell_ptr;

        GcPointer::new(unsafe { &*cell_ptr })
    }

    pub fn is_empty(&self) -> bool {
        self.used_cell.is_null()
    }

    pub fn has_empty_slot(&self) -> bool {
        !self.free_cell.is_null()
    }

    pub fn sweep(&mut self, sweep_type: SweepType) {
        let mut prev_ptr = std::ptr::null_mut() as *mut Cell;
        let mut cell_ptr = self.used_cell;
        loop {
            if cell_ptr.is_null() {
                break;
            }

            let cell = unsafe { cell_ptr.as_mut().unwrap() };
            if !cell.is_marked() || sweep_type == SweepType::Everything {
                let next_ptr = cell.next;

                unsafe {
                    // Clean up old cell
                    std::ptr::drop_in_place(cell.get_dyn());
                    Cell::init_free(cell_ptr, self.free_cell);
                    self.free_cell = cell_ptr;
                };

                if !prev_ptr.is_null() {
                    unsafe { (*prev_ptr).next = next_ptr }
                } else {
                    self.used_cell = next_ptr;
                }

                cell_ptr = next_ptr;
            } else {
                cell.unmark();
                prev_ptr = cell_ptr;
                cell_ptr = cell.next;
            }
        }
    }

    pub fn cell_size(&self) -> usize {
        self.cell_size
    }

    pub fn recycle(&mut self, cell_size: usize) {
        debug_assert!(self.is_empty());
        debug_assert!(cell_size.is_power_of_two());
        debug_assert!(BLOCK_SIZE % cell_size == 0);

        let num_cell = BLOCK_SIZE / cell_size;

        let mut cell = self.ptr as *mut Cell;
        for _ in 0..num_cell {
            unsafe {
                let next = (cell as *mut u8).add(cell_size) as *mut Cell;
                Cell::init_free(cell, next);
                cell = next;
            }
        }

        self.cell_size = cell_size;
        self.free_cell = self.ptr as *mut Cell;
        self.used_cell = std::ptr::null_mut();
    }
}

impl Drop for HeapBlock {
    fn drop(&mut self) {
        if !self.is_empty() {
            panic!(
                "Dropping non-empty HeapBlock: cell_size={}, block_size={}",
                self.cell_size, BLOCK_SIZE
            )
        }

        let layout = std::alloc::Layout::from_size_align(BLOCK_SIZE, self.cell_size).unwrap();
        unsafe { std::alloc::dealloc(self.ptr, layout) }
    }
}
