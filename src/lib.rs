//! LLVM bindings for Rust

pub use self::upcast::Upcast;

#[macro_use]
pub mod upcast;
pub mod ir;
pub mod target;
pub mod pass;

/// The C FFI library.
extern crate llvm_sys as sys;
#[macro_use]
extern crate lazy_static;
