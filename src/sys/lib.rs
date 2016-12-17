pub use self::ir::*;

pub mod ir;

extern crate libc;

// Required to link LLVM
extern crate ncurses;
// Required to link LLVM
#[link(name = "ffi")] extern {}

#[cfg(test)]
pub mod test_support;

pub enum OpaqueContext { }
pub type ContextRef = *mut OpaqueContext;

pub enum OpaqueType { }
pub type TypeRef = *mut OpaqueType;

pub enum OpaqueValue { }
pub type ValueRef = *mut OpaqueValue;
