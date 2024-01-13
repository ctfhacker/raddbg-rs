//! Operating system specific

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

/// Reserve a memory region of the given size
pub fn reserve(size: u64) -> *mut u8 {
    #[cfg(target_os = "windows")]
    let func = windows::reserve;

    #[cfg(target_os = "linux")]
    let func = linux::reserve;

    func(size)
}

/// Change the protections to RW for the given allocation
pub fn commit(ptr: *const u8, size: u64) -> u32 {
    #[cfg(target_os = "windows")]
    let func = windows::commit;

    #[cfg(target_os = "linux")]
    let func = linux::commit;

    func(ptr, size)
}
