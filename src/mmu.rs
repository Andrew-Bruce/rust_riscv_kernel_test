use crate::memory_alloc;

pub mod sv39_page_table;

pub fn map_range(
    root: &mut sv39_page_table::Sv39PageTable,
    start: usize,
    end: usize,
    protection_bits: u8,
) {
    let paddr_start: usize = memory_alloc::align(start, memory_alloc::PAGE_SIZE);
    let paddr_end: usize = memory_alloc::align(end, memory_alloc::PAGE_SIZE);

    for paddr in paddr_start..paddr_end {
        let vaddr = paddr; //since mapping kernel memory to itself
        sv39_page_table::create_virtual_to_physical_mapping(root, vaddr, paddr, protection_bits, 0);
    }
}
