//use crate::memory_alloc;

pub mod sv39;

pub fn memory_map_region(start: usize, end: usize, root_table: &mut sv39::PageTable) {
    for addr in start..end {
        sv39::map(addr, addr, root_table).unwrap();
    }
}
