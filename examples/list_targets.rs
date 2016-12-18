extern crate llvm;

use llvm::target;

fn main() {
    for target in target::Registry::get().targets() {
        println!("{} - {}", target.name(), target.short_description());
    }
}
