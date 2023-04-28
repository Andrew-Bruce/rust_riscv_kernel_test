

#[repr(u8)]
pub enum PageBits {
    Empty = 0,
    Taken = 1 << 0,
    Last = 1 << 1,
}

pub struct Page {
    flags: u8,
}


pub fn allocate_pages(num_pages: usize){
    assert!(0 <= num_pages);
    let total_pages = unsafe{ crate::HEAP_SIZE};
}
