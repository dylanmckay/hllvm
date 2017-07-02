#![allow(improper_ctypes)]

pub use self::ir::*;
pub use self::target::*;
pub use self::support::*;
pub use self::ffi_helpers::*;

pub mod ir;
pub mod target;
pub mod support;
pub mod ffi_helpers;

extern crate libc;
#[macro_use]
extern crate cpp;

// Required to link LLVM
extern crate ncurses;
// Required to link LLVM
#[link(name = "ffi")] extern { }

#[cfg(test)]
pub mod test_support;

macro_rules! define_borrowed_type {
    ($name:ident, $opaque_name:ident) => {
        pub enum $opaque_name { }
        pub type $name = *mut $opaque_name;
    }
}

/// We don't know the sizes of many LLVM types, and they can change.
///
/// To work around this, we box up values from C++ and deal with the boxed
/// pointers in Rust, and then dereference to get back to the inner value in C++ again.
macro_rules! define_boxed_type {
    ($name:ident, $opaque_name:ident) => {
        #[derive(Clone)]
        pub enum $opaque_name { }
        pub type $name = CppBox<$opaque_name>;
    }
}

define_borrowed_type!(ContextRef, OpaqueContext);
define_borrowed_type!(TypeRef, OpaqueType);
define_borrowed_type!(ValueRef, OpaqueValue);
define_borrowed_type!(TargetRef, OpaqueTarget);

define_borrowed_type!(TargetMachineRef, OpaqueTargetMachine);
define_borrowed_type!(PassManagerRef, OpaquePassManager);
define_borrowed_type!(RawPWriteStreamRef, OpaqueRawPWriteStream);

define_boxed_type!(AttributeRef, OpaqueAttribute);

