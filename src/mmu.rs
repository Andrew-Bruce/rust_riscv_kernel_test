use core::arch::asm;
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

pub fn enable_mmu(root_table_ptr: *const sv39::PageTable) {
    let root_table_ppn: usize = root_table_ptr as usize >> 12;
    let satp_val: usize = (8 << 60) | root_table_ppn;
    unsafe { asm!("csrw satp, {}", in(reg) satp_val) }
}
