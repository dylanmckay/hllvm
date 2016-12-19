//! LLVM bindings for Rust

pub use self::upcast::Upcast;
pub use self::safe_wrapper::SafeWrapper;

#[macro_use]
pub mod upcast;
pub mod ir;
pub mod target;
pub mod pass;
pub mod support;
pub mod safe_wrapper;

/// The C FFI library.
extern crate llvm_sys as sys;
#[macro_use]
extern crate lazy_static;
extern crate libc;
