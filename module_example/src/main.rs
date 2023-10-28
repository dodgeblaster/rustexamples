mod a;
mod b;
mod utils {
    pub mod core;
}

fn main() {
    println!("Hello, world!");
    a::function();
    b::function();
    utils::core::function();
}
