extern crate hllvm;

use hllvm::target;

fn main() {
    println!("Default triple: {}\n", target::Triple::default());

    for target in target::Registry::get().targets() {
        println!("{} - {}", target.name(), target.short_description());
    }
}
