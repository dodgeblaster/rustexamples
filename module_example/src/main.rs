mod a;
mod b;
mod utils {
    pub mod core;
}

use utils::core as core;

fn main() {
    println!("Hello, world!");
    a::function();
    b::function();
    core::function();
}
