pub use self::block::Block;

pub mod block;

use sys;

use std::marker;

/// An LLVM value.
pub struct Value<'ctx>
{
    inner: sys::ValueRef,
    phantom: marker::PhantomData<&'ctx ()>,
}

impl<'ctx> Value<'ctx>
{
    /// Creates a value from a reference to an LLVM value.
    pub fn new(inner: sys::ValueRef) -> Self {
        Value { inner: inner, phantom: marker::PhantomData }
    }

    /// Dumps the value to standard error.
    pub fn dump(&self) {
        unsafe {
            sys::LLVMRustValueDump(self.inner);
        }
    }

    /// Gets the underlying reference to the value.
    pub fn inner(&self) -> sys::ValueRef { self.inner }
}
