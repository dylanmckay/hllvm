extern crate hllvm;

use hllvm::target;

fn main() {
    for target in target::Registry::get().targets() {
        println!("{} - {}", target.name(), target.short_description());
    }
}
