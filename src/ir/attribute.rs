use ir::Context;
use sys;

use std::marker;
use std::ffi;

/// An LLVM attribute.
/// `'ctx` - the lifetime of the context.
pub struct Attribute<'ctx>
{
    inner: sys::AttributeRef,
    phantom: marker::PhantomData<&'ctx ()>,
}

impl<'ctx> Attribute<'ctx>
{
    /// Creates a new integer-valued attribute.
    /// TODO: create an enum for `kind`
    pub fn integer(kind: u32,
                   value: u64,
                   context: &Context) -> Self {
        let inner = unsafe {
            sys::LLVMRustAttributeGetInteger(context.inner(), kind as _, value)
        };
        Attribute::new(inner)
    }

    /// Creates a new string-valued attribute.
    pub fn string(kind: &str,
                  value: &str,
                  context: &Context) -> Self {
        let kind = ffi::CString::new(kind).unwrap();
        let value = ffi::CString::new(value).unwrap();

        let inner = unsafe {
            sys::LLVMRustAttributeGetString(context.inner(), kind.as_ptr(), value.as_ptr())
        };
        Attribute::new(inner)
    }

    /// Gets the inner attribute reference.
    pub fn inner(&self) -> sys::AttributeRef { self.inner.clone() }

    fn new(inner: sys::AttributeRef) -> Self {
        Attribute { inner: inner, phantom: marker::PhantomData }
    }
}
