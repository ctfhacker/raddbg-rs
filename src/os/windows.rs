//! Windows specific operations

extern "C" {
    pub fn VirtualAlloc(addr: *const u8, size: u64, alloc_type: u32, prot: u32) -> *mut u8;
}

const MEM_COMMIT:  u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const PAGE_READWRITE: u32 = 0x4;

pub fn reserve(size: u64) -> *mut u8 {
    unsafe {
        VirtualAlloc(std::ptr::null(), size, MEM_RESERVE, PAGE_READWRITE)
    }
}

pub fn commit(ptr: *const u8, size: u64) -> u32 {
    let res = unsafe {
        VirtualAlloc(ptr, size as u64, MEM_COMMIT, PAGE_READWRITE)
    };

    (res as usize != 0) as u32
}
