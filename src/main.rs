//! LLVM bindings for Rust

pub mod ir;

/// The C FFI library.
extern crate llvm_sys as sys;

fn main() {
    let context = ir::Context::new();
    let module = ir::Module::new("mymodule", &context);
    module.dump();

    ir::Type::integer(8, &context).dump();
}
