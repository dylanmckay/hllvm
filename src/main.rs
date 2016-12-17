//! LLVM bindings for Rust

pub mod ir;

/// The C FFI library.
extern crate llvm_sys as sys;

fn main() {
    let context = ir::Context::new();
    let module = ir::Module::new("mymodule", &context);

    let int8 = ir::IntegerType::integer(8, &context);

    let func_ty = ir::FunctionType::new(&int8.as_ref(), &[], false);
    let func = module.get_or_insert_function("my_func", &func_ty, &[]);

    module.dump();
}
