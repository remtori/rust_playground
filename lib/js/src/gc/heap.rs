use super::Tracer;
use super::{BLOCK_SIZE, SweepType, HeapBlock, Cell, GcPointer, GcCell};
use std::collections::HashSet;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

pub struct Heap {
    map_size_to_blocks: BTreeMap<usize, Vec<HeapBlock>>,
    root_collector: Vec<Box<dyn FnMut(&mut HashSet<*mut GcCell>)>>,
}

impl Heap {
    pub fn new(cell_sizes: &[usize], block_size: usize) -> Heap {
        debug_assert!(cell_sizes.iter().all(|s| *s > std::mem::size_of::<Cell>()));
        debug_assert!(cell_sizes.iter().all(|s| s.is_power_of_two()));
        debug_assert!(cell_sizes.iter().all(|s| block_size % s == 0));

        let mut heap = Heap {
            map_size_to_blocks: BTreeMap::new(),
            root_collector: Vec::new(),
        };

        for cell_size in cell_sizes {
            heap.allocate_block(*cell_size);
        }

        heap
    }

    pub fn add_root_collector<F>(&mut self, func: F)
    where F: FnMut(&mut HashSet<*mut GcCell>) + 'static,
    {
        self.root_collector.push(Box::new(func));
    }

    pub fn allocate<T>(&mut self, obj: T) -> GcPointer<T>
    where
        T: Sized + 'static + GcCell,
    {
        let cell_size = usize::next_power_of_two(std::mem::size_of_val(&obj));
        match self.get_block_mut(cell_size) {
            Some(block) => block.allocate(obj),
            None => self.allocate_block(cell_size).allocate(obj),
        }
    }

    pub fn collect_garbage(&mut self, sweep_type: SweepType) {
        if sweep_type == SweepType::Everything {
            self.sweep(sweep_type);
            return;
        }

        let roots = &mut HashSet::new();
        for collector in self.root_collector.iter_mut() {
            collector(roots);
        }

        let tracer = &mut Tracer::new();
        for root in roots.iter() {
            unsafe { (**root).trace(tracer) };
        }

        self.sweep(sweep_type)
    }

    pub fn sweep(&mut self, sweep_type: SweepType) {        
        for block_list in self.map_size_to_blocks.values_mut() {
            for block in block_list {
                block.sweep(sweep_type)
            }
        }

        if sweep_type == SweepType::Garbage {   
            self.map_size_to_blocks.retain(|_, block_list| {
                block_list.retain(|block| !block.is_empty());
                !block_list.is_empty()
            });
        }
    }

    fn allocate_block(&mut self, cell_size: usize) -> &mut HeapBlock {
        debug_assert!(cell_size.is_power_of_two());
        debug_assert!(BLOCK_SIZE % cell_size == 0);

        let blocks = match self.map_size_to_blocks.entry(cell_size) {
            Entry::Vacant(v) => v.insert(Vec::new()),
            Entry::Occupied(o) => o.into_mut(),
        };

        blocks.push(HeapBlock::new(cell_size));
        blocks.last_mut().unwrap()
    }

    fn get_block_mut(&mut self, cell_size: usize) -> Option<&mut HeapBlock> {
        if let Some(blocks) = self.map_size_to_blocks.get_mut(&cell_size) {
            for block in blocks {
                if block.has_empty_slot() {
                    return Some(block);
                }
            }
        }

        None
    }
}

impl Default for Heap {
    #[rustfmt::skip]
    fn default() -> Self {
        Heap::new(&[
                32, 32, 32, 32, 
                64, 64, 64, 64, 
                256, 256, 
                1024
            ], 
            4 * 1024
        )
    }
}
