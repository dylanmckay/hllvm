pub use self::ir::*;

pub mod ir;

// Required to link LLVM
extern crate ncurses;
// Required to link LLVM
#[link(name = "ffi")] extern {}

#[cfg(test)]
pub mod test_support;
