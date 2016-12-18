pub use self::ir::*;
pub use self::target::*;

pub mod ir;
pub mod target;

extern crate libc;

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

define_boxed_type!(AttributeRef, OpaqueAttribute);

/// An array passed from Rust to C.
///
/// This is useful because that way we don't have to pass heaps of
/// `ptr` and `count` fields to C++.
#[repr(C, packed)]
pub struct Array<T>
{
    ptr: *mut T,
    count: libc::size_t,
}

impl<T> Array<T>
{
    pub fn from_slice(slice: &mut [T]) -> Self {
        Array {
            ptr: slice.as_mut_ptr(),
            count: slice.len(),
        }
    }
}

/// A boxed value, allocated from C++.
/// We own the value and free it ourselves.
#[repr(C, packed)]
pub struct CppBox<T>
{
    ptr: *mut T,
}

impl<T> Drop for CppBox<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.ptr as *mut libc::c_void) }
    }
}

// We don't allow copying because that could lead to double-frees.
impl<T: Clone> Clone for CppBox<T> {
    fn clone(&self) -> Self {
        use std::mem::size_of;
        use std::ptr;

        unsafe {
            let new_ptr = libc::malloc(size_of::<T>()) as *mut T;
            ptr::copy(self.ptr, new_ptr, size_of::<T>());

            CppBox { ptr: new_ptr }
        }
    }
}
