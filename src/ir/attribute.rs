//! An attribute which can be placed on certain IR data structures.

use SafeWrapper;
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
    /// FIXME: create an enum for `kind`
    pub fn integer(kind: u32,
                   value: u64,
                   context: &Context) -> Self {
        unsafe {
            let inner = sys::LLVMRustAttributeGetInteger(context.inner(), kind as _, value);
            Attribute::from_inner(inner)
        }
    }

    /// Creates a new string-valued attribute.
    pub fn string(kind: &str,
                  value: &str,
                  context: &Context) -> Self {
        let kind = ffi::CString::new(kind).unwrap();
        let value = ffi::CString::new(value).unwrap();

        unsafe {
            let inner = sys::LLVMRustAttributeGetString(context.inner(), kind.as_ptr(), value.as_ptr());
            Attribute::from_inner(inner)
        }
    }
}

impl<'ctx> SafeWrapper for Attribute<'ctx>
{
    type Inner = sys::AttributeRef;

    unsafe fn from_inner(inner: sys::AttributeRef) -> Self {
        Attribute { inner: inner, phantom: marker::PhantomData }
    }

    fn inner(&self) -> sys::AttributeRef { self.inner.clone() }
}
