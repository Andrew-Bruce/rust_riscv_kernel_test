use crate::println;
use crate::HEAP_END;
use crate::HEAP_SIZE;
use crate::HEAP_START;

static mut ALLOC_START: usize = 12345;
pub const PAGE_SIZE: usize = 4096;

#[repr(u8)]
#[derive(Clone, Copy)]
enum PageBits {
    Empty = 0,
    Taken = 1 << 0,
    Last = 1 << 1,
}

#[derive(Clone, Copy)]
struct Page {
    flags: u8,
}

impl PageBits {
    fn byte(&self) -> u8 {
        *self as u8
    }
}

impl Page {
    fn is_free(&self) -> bool {
        self.flags == PageBits::Empty.byte()
    }

    fn is_taken(&self) -> bool {
        (self.flags & PageBits::Taken.byte()) != 0
    }

    fn is_last(&self) -> bool {
        (self.flags & PageBits::Last.byte()) != 0
    }

    fn clear(&mut self) {
        self.flags = PageBits::Empty.byte();
    }

    fn mark_taken(&mut self) {
        self.flags |= PageBits::Taken.byte();
    }
    fn mark_last(&mut self) {
        self.flags |= PageBits::Last.byte();
    }
}

pub fn allocate_pages(num_pages: usize) -> Option<*mut u8> {
    let total_pages: usize = unsafe { HEAP_SIZE } / PAGE_SIZE;

    let heap_start_page: *mut Page = unsafe { HEAP_START } as *mut Page;

    let mut found: bool = false;

    for start_page_index in 0..(total_pages - num_pages) {
        let start: Page = unsafe { *heap_start_page.add(start_page_index) };

        if start.is_free() {
            found = true;

            for next_page_index in start_page_index..(start_page_index + num_pages) {
                unsafe {
                    let curr: *mut Page = heap_start_page.add(next_page_index);

                    if (*curr).is_taken() {
                        found = false;
                        break;
                    }
                }
            }
        }

        if found {
            for next_page_index in start_page_index..(start_page_index + num_pages) {
                unsafe {
                    let curr: *mut Page = heap_start_page.add(next_page_index);
                    assert!(!(*curr).is_taken());
                    assert!(!(*curr).is_last());
                    (*curr).mark_taken();
                    assert!((*curr).is_taken());
                }
            }
            unsafe {
                let last: *mut Page = heap_start_page.add(start_page_index + num_pages - 1);
                assert!(!(*last).is_last());
                assert!((*last).is_taken());
                (*last).mark_last();
                assert!((*last).is_last());
                println!(
                    "ALLOCED MEMORY pages FROM {:p} to {:p}",
                    heap_start_page.add(start_page_index),
                    last
                );
            };
            return Some((unsafe { ALLOC_START } + (PAGE_SIZE * start_page_index)) as *mut u8);
        }
    }
    None
}

pub fn zero_allocate_pages(num_pages: usize) -> Option<*mut u8> {
    let memory: *mut u8 = allocate_pages(num_pages)?;
    let size: usize = num_pages * PAGE_SIZE;

    for i in 0..size {
        unsafe {
            memory.add(i).write_volatile(0);
        }
    }

    return Some(memory);
}

pub fn deallocate_pages(start_ptr: *mut u8) {
    assert!(!start_ptr.is_null());
    let addr: usize =
        unsafe { HEAP_START } + ((start_ptr as usize - unsafe { ALLOC_START }) / PAGE_SIZE);
    assert!((unsafe { HEAP_START } <= addr) && (addr < unsafe { HEAP_END }));
    let mut start_page: *mut Page = addr as *mut Page;

    unsafe {
        while (*start_page).is_taken() && !(*start_page).is_last() {
            (*start_page).clear();
            start_page = start_page.add(1);
        }
    }
    assert!(unsafe { (*start_page).is_last() });
    unsafe { (*start_page).clear() };
}

pub fn print_page_allocation() {
    let total_pages = unsafe { HEAP_SIZE / PAGE_SIZE };
    let page_data_begin = unsafe { HEAP_START };
    let page_data_end = unsafe { (page_data_begin as *const Page).add(total_pages) } as usize;

    println!("page size   = {}", PAGE_SIZE);
    println!("total pages = {}", total_pages);
    println!(
        "page data  size = {:#010x}",
        page_data_end - page_data_begin
    );
    println!("page alloc size = {:#010x}", total_pages * PAGE_SIZE);
    println!(
        "page data   | {:#010x} -> {:#010x}",
        page_data_begin, page_data_end
    );
    println!(
        "pages alloc | {:#010x} -> {:#010x}",
        unsafe { ALLOC_START },
        unsafe { ALLOC_START } + total_pages * PAGE_SIZE
    );

    let first_page: *mut Page = page_data_begin as *mut Page;
    let mut curr_in_page: bool = false;
    let mut start: usize = 0;
    let mut num_pages: u32 = 0;
    for page_index in 0..total_pages {
        let curr: *mut Page = unsafe { first_page.add(page_index) };

        let curr_is_taken: bool = unsafe { *curr }.is_taken();
        let curr_is_last: bool = unsafe { *curr }.is_last();
        let curr_is_free: bool = unsafe { *curr }.is_free();

        assert!(curr_is_taken ^ curr_is_free);

        if curr_in_page {
            assert!(curr_is_taken);
            num_pages += 1;
        } else if curr_is_taken {
            num_pages = 1;
            curr_in_page = true;
            start = curr as usize;
        }
        if curr_is_last {
            assert!(curr_in_page);
            assert!(curr_is_taken);
            curr_in_page = false;
            let end = curr as usize;
            println!(
                "Page {:#010x} -> {:#010x} ({} pages)",
                start, end, num_pages
            );
        }
    }
    assert!(!curr_in_page);
}

pub fn init() {
    let num_pages: usize = unsafe { HEAP_SIZE } / PAGE_SIZE;
    let first_page: *mut Page = unsafe { HEAP_START } as *mut Page;

    for page_index in 0..num_pages {
        let page: *mut Page = unsafe { first_page.add(page_index) };
        unsafe {
            (*page).clear();
            assert!((*page).is_free());
            assert!(!(*page).is_taken());
            assert!(!(*page).is_last());
        }
    }

    unsafe { ALLOC_START = HEAP_START + (num_pages * core::mem::size_of::<Page>()) };

    //align
    let align_to = PAGE_SIZE;

    unsafe { ALLOC_START += align_to - (ALLOC_START % align_to) };
    assert!(unsafe { ALLOC_START } % align_to == 0);
}
