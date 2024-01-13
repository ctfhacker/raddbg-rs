//! Linux specific operations

extern "C" {
    fn mmap(addr: *const u8, length: usize, prot: i32, flags: i32, fd: i32, offset: i64)
        -> *mut u8;

    fn mprotect(addr: *const u8, length: usize, prot: i32) -> u32;
}

const PROT_NONE: i32 = 0x0;
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const MAP_PRIVATE: i32 = 0x02;
const MAP_ANON: i32 = 0x20;
const MAP_FAILED: isize = -1;

pub fn reserve(size: u64) -> *mut u8 {
    let res = unsafe {
        mmap(
            std::ptr::null(),
            size as usize,
            PROT_NONE,
            MAP_PRIVATE | MAP_ANON,
            -1,
            0,
        )
    };

    assert!(
        (res as isize) != MAP_FAILED,
        "Linux reserved failed to allocate {size}"
    );

    res
}

pub fn commit(ptr: *const u8, size: u64) -> u32 {
    let result = unsafe { mprotect(ptr, size as usize, PROT_READ | PROT_WRITE) };
    (result == 0) as u32
}
