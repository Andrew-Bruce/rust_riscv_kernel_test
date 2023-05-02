// Riscv Sv39 page table implementation
// go to the riscv website and find the privileged ISA pdf (volume 2)
// then there is a section explaining Sv39
// i don't really understand it that well ngl

use crate::memory_alloc;

const PAGE_TABLE_NUM_ENTRIES: usize = 512;
const NUM_LEVELS: isize = 3;

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PteBits {
    Valid = 1 << 0,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    UserMode = 1 << 4, // User Mode can access
    Globe = 1 << 5,    // Global mapping
    Accessed = 1 << 6,
    Dirty = 1 << 7,
}

impl PteBits {
    fn val(&self) -> usize {
        *self as usize
    }
}

struct VirtAddr {
    bits: usize,
}

struct Pte {
    bits: usize,
}

pub struct PageTable {
    entries: [Pte; PAGE_TABLE_NUM_ENTRIES],
}

impl VirtAddr {
    fn get_vpn(&self) -> [usize; 3] {
        let nine_ones: usize = 0b1_1111_1111;
        [
            (self.bits >> 12) & nine_ones,
            (self.bits >> 21) & nine_ones,
            (self.bits >> 30) & nine_ones,
        ]
    }

    fn get_whole_vpn(&self) -> usize {
        let twenty_seven_ones = 0x7ff_ffff;
        (self.bits >> 12) | twenty_seven_ones
    }

    fn get_offset(&self) -> usize {
        let twelve_ones: usize = 0xfff;

        self.bits & twelve_ones
    }
}

impl Pte {
    fn new(ppn: usize, protection_bits: usize) -> Pte {
        assert!(protection_bits < (1 << 8));

        let out = Pte {
            bits: (ppn << 10) & protection_bits,
        };
        out.assert_not_reserved();
        out
    }

    fn get_ppn(&self) -> [usize; 3] {
        let nine_ones: usize = 0b1_1111_1111;
        let twenty_six_ones: usize = 0x3ff_ffff;
        [
            (self.bits >> 10) & nine_ones,
            (self.bits >> 19) & nine_ones,
            (self.bits >> 28) & twenty_six_ones,
        ]
    }

    fn get_whole_ppn(&self) -> usize {
        let fourty_four_ones: usize = 0xfff_ffff_ffff;
        (self.bits >> 10) & fourty_four_ones
    }

    fn get_physical_addr(&self) -> *mut u8 {
        let ppn: usize = self.get_whole_ppn();
        (ppn << 12) as *mut u8
    }

    fn is_valid(&self) -> bool {
        self.bits & PteBits::Valid.val() != 0
    }

    fn is_read(&self) -> bool {
        self.bits & PteBits::Read.val() != 0
    }
    fn is_write(&self) -> bool {
        self.bits & PteBits::Write.val() != 0
    }
    fn is_execute(&self) -> bool {
        self.bits & PteBits::Execute.val() != 0
    }

    fn is_branch(&self) -> bool {
        !(self.is_read() || self.is_write() || self.is_execute())
    }

    fn is_leaf(&self) -> bool {
        !self.is_branch()
    }

    fn is_reserved(&self) -> bool {
        self.is_write() && !self.is_read()
    }

    fn assert_not_reserved(&self) {
        assert!(!self.is_reserved());
    }
}

fn virt_to_phys_rec(va: VirtAddr, root: &PageTable, depth: isize) -> Result<usize, &str> {
    if depth < 0 {
        return Err("depth reached negative before leaf found");
    }
    let vpn: [usize; 3] = va.get_vpn();
    let curr_pte: &Pte = &root.entries[vpn[depth as usize]];

    if !curr_pte.is_valid() {
        return Err("hit invalid page");
    }
    curr_pte.assert_not_reserved();

    if curr_pte.is_branch() {
        let new_table: &PageTable = unsafe {
            (curr_pte.get_physical_addr() as *const PageTable)
                .as_ref()
                .unwrap()
        };
        return virt_to_phys_rec(va, new_table, depth - 1);
    }

    assert!(curr_pte.is_leaf());

    if depth > 0 {
        //only needed if doing more than 4kb pages
        let ppn: [usize; 3] = curr_pte.get_ppn();
        for i in 0..depth {
            if ppn[i as usize] != 0 {
                return Err("misaligned superpage");
            }
        }
    }

    let vpn_mask: usize = (1 << (9 * depth)) - 1;

    let ppn_bits = curr_pte.get_whole_ppn();
    let vpn_bits = va.get_whole_vpn() & vpn_mask;
    assert!(ppn_bits & vpn_bits == 0);
    let page_number: usize = ppn_bits | vpn_bits;
    let page_addr = page_number << 12;
    let page_offset = va.get_offset();
    assert!(page_addr & page_offset == 0);
    Ok(page_addr | page_offset)
}

pub fn virt_to_phys(va: usize, root: &PageTable) -> Result<*mut u8, &str> {
    let va = VirtAddr { bits: va };

    let out = virt_to_phys_rec(va, root, NUM_LEVELS - 1)?;

    Ok(out as *mut u8)
}

fn map_rec(
    va: VirtAddr,
    pa: usize,
    root: &mut PageTable,
    target_depth: usize,
    curr_depth: isize,
) -> Result<(), &str> {
    assert!(curr_depth >= 0);
    assert!((curr_depth as usize) >= target_depth);
    let vpn: [usize; 3] = va.get_vpn();

    let pte = &mut root.entries[vpn[curr_depth as usize]];

    if (curr_depth as usize) == target_depth {
        *pte = Pte::new(pa << 12, PteBits::Valid.val());
        return Ok(());
    }

    if !pte.is_valid() {
        let new_page: *mut u8 = memory_alloc::zero_allocate_pages(1).unwrap();
        let new_entry = Pte::new(new_page as usize, PteBits::Valid.val());
        *pte = new_entry;
        assert!(core::ptr::eq(pte.get_physical_addr(), new_page));
    }

    let next_table = unsafe {
        (pte.get_physical_addr() as *mut PageTable)
            .as_mut()
            .unwrap()
    };

    return map_rec(va, pa, next_table, target_depth, curr_depth - 1);
}

pub fn map(va: usize, pa: usize, root: &mut PageTable) -> Result<(), &str> {
    return map_rec(VirtAddr { bits: va }, pa, root, 2, 2);
}
