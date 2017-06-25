//! LLVM bindings for Rust

pub use self::subtype::Subtype;
pub use self::safe_wrapper::SafeWrapper;

#[macro_use]
pub mod subtype;
pub mod ir;
pub mod target;
pub mod pass;
pub mod support;
pub mod safe_wrapper;

/// The C FFI library.
extern crate hllvm_sys as sys;
#[macro_use]
extern crate lazy_static;
extern crate libc;
