//use crate::memory_alloc;
pub mod sv39;

pub fn memory_map_region(
    start: usize,
    end: usize,
    root_table: &mut sv39::PageTable,
    protection_bits: usize,
) {
    assert!(protection_bits < (1 << 8));
    assert!(start % 4096 == 0);
    //assert!(end % 4096 == 0);
    for addr in (start..end).step_by(4096) {
        sv39::map(addr, addr, root_table, protection_bits).unwrap();
    }
}
