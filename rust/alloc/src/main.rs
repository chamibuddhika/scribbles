extern crate alloc;

fn main() {
    let a = Box::new(8); // allocates memory via our custom allocator crate
    println!("{}", a);
}
