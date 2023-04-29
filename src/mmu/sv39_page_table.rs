// Riscv Sv39 page table implementation
// go to the riscv website and find the privileged ISA pdf (volume 2)
// then there is a section explaining Sv39
// i don't really understand it that well ngl

use crate::memory_alloc::PAGE_SIZE;

const PAGE_TABLE_NUM_ENTRIES: usize = 512;

#[repr(usize)]
#[derive(Clone, Copy)]
enum Sv39PageTableEntryBits {
    V = 1 << 0, // Valid
    R = 1 << 1, // Read
    W = 1 << 2, // Write
    X = 1 << 3, // Execute
    U = 1 << 4, // User Mode can access
    G = 1 << 5, // Global mapping
    A = 1 << 6, // Accessed
    D = 1 << 7, // Dirty
}

impl Sv39PageTableEntryBits {
    fn bits(&self) -> usize {
        *self as usize
    }
}

struct Sv39VirtualAddress {
    // VPN means "virtual page number", which is the index into the page table
    // each VPN is 9 bits so can index up to 512 pages
    // Total of 39 bits, so where the name Sv39 comes from

    // bits[0..12] = page offset
    // bits[12..21] = VPN[0]
    // bits[21..30] = VPN[1]
    // bits[30..39] = VPN[2]
    bits: usize,
}

//A Page table should take up exactly one page and it has to be page aligned or the MMU messes up
static_assertions::const_assert_eq!(PAGE_SIZE, core::mem::size_of::<Sv39PageTable>());
struct Sv39PageTableEntry {
    // PPN means physical page number, an address in physical memory 
    // which is just a regular pointer for the kernel

    // bits[0..8]   = [V, R, W, X, U, G, A, D]
    // bits[8..10]  = RSW      //ignore this field, only used by supervisor
    // bits[10..19] = PPN[0]
    // bits[19..28] = PPN[1]
    // bits[29..54] = PPN[2]
    // bits[54..61] = RESERVED //reserved for future use, should ALWAYS be zeros
    // bits[61..63] = PBMT     //used for the Svpbmt  extension, if no extension then ALWAYS be zeros
    // bits[63]     = N        //used for the Svnapot extention, if no extension then ALWAYS be zero
    bits: usize,
    // to get the physical address we convert it into
    // PPN[2] ++ PPN[1] ++ PPN[0] ++ page offset
    // 26 bits + 9 bits +  9 bits + 12 bits
    // = 56 bit physical address
    // so we can't access all 2^64 of hypothetical memory but in reality that shouldn't matter
    // since there is never that much

    // since the Virtual address has 3 VPNs, a page table entry can point to other page tables
    // R, W, X all being 0 means that it is a branch and points to another page table
    // otherwise it is a leaf and points to usable memory

    // | X | W | R | meaning
    // |---|---|---|-------------
    // | 0 | 0 | 0 | pointer to next level of page table
    // | 0 | 0 | 1 | R-- page
    // | 0 | 1 | 0 | RESERVED
    // | 0 | 1 | 1 | RW- page
    // | 1 | 0 | 0 | --X page
    // | 1 | 0 | 1 | R-X page
    // | 1 | 1 | 0 | RESERVED
    // | 1 | 1 | 1 | RWX page
}

impl Sv39VirtualAddress {
    fn get_vpns(&self) -> [usize; 3] {
        //see the struct for the layout
        let nine_ones: usize = 0b111111111;
        return [
            (self.bits >> 12) & nine_ones,
            (self.bits >> 21) & nine_ones,
            (self.bits >> 30) & nine_ones,
        ];
    }
}


impl Sv39PageTableEntry {
    fn new(protection_bits: u8, physical_addr: *const u8) -> Sv39PageTableEntry{
        let ppn: usize = (physical_addr as usize) >> 12;
        Sv39PageTableEntry{
            bits: usize::from(protection_bits) | (ppn << 10),
        }
    }

    fn is_valid(&self) -> bool {
        self.bits & Sv39PageTableEntryBits::V.bits() != 0
    }
    fn can_read(&self) -> bool {
        self.bits & Sv39PageTableEntryBits::R.bits() != 0
    }
    fn can_write(&self) -> bool {
        self.bits & Sv39PageTableEntryBits::W.bits() != 0
    }
    fn can_execute(&self) -> bool {
        self.bits & Sv39PageTableEntryBits::X.bits() != 0
    }
}

struct Sv39PageTable {
    entries: [Sv39PageTableEntry; PAGE_TABLE_NUM_ENTRIES],
}

fn create_virtual_to_physical_mapping(root: &mut Sv39PageTable, vaddr: usize, paddr: usize, protection_bits: u8, level: usize) {
    
    let vpn: [usize; 3] = (Sv39VirtualAddress { bits: vaddr }).get_vpns();

    let nine_ones: usize = 0b111111111;
    let twenty_six_ones: usize = 0x3ff_ffff;
    let ppn: [usize; 3] = 
        [
            (paddr >> 12) & nine_ones,
            (paddr >> 21) & nine_ones,
            (paddr >> 30) & twenty_six_ones,
        ]; 
    
    let new_table: *mut u8 = crate::memory_alloc::allocate_pages(1).unwrap();

    let mut v = &mut root.entries[vpn[2]];

    for i in (level..2).rev() {
        let protection_bits: u8 = Sv39PageTableEntryBits::V.bits() as u8;
        *v = Sv39PageTableEntry::new(protection_bits, new_table);
    }
}
