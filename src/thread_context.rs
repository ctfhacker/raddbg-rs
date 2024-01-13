use crate::arena::Arena;

thread_local! {
    pub static tctx_thread_local = Tctx::new();
}

/// Thread context
pub struct Tctx {
    arenas: [Arena; 2],

    thread_name: [u8; 32],
    thread_name_size: u64,

    file_name: &'static str,
    lien_number: u64
}

pub fn Tctx {
    pub fn new() -> Tctx {

        
    }
}
