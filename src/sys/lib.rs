pub mod ir;

// Required to link LLVM
extern crate ncurses;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

// Required to link LLVM
#[link(name = "ffi")] extern {}
