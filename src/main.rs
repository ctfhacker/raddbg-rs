mod arena;
mod os;

fn main() {
    let ptr = os::reserve(0x1000);
    println!("Ptr: {ptr:p}");
}
