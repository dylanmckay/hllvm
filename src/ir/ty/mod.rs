pub use self::func::FunctionType;
pub use self::integer::IntegerType;
pub use self::composite::*;

pub mod func;
pub mod integer;
pub mod composite;

use ir::Context;
use sys;

use std::marker;

/// An LLVM type.
pub struct Type<'ctx>
{
    inner: sys::TypeRef,
    phantom: marker::PhantomData<&'ctx ()>,
}

impl<'ctx> Type<'ctx>
{
    /// Builds a type from a reference to an `llvm::Type` object.
    pub unsafe fn new(inner: sys::TypeRef) -> Self {
        Type { inner: inner, phantom: marker::PhantomData }
    }

    /// Gets the `void` type.
    pub fn void(context: &Context) -> Self {
        unsafe { Type::new(sys::LLVMRustTypeGetVoidTy(context.inner())) }
    }

    /// Dump the type to standard error.
    pub fn dump(&self) {
        unsafe { sys::LLVMRustTypeDump(self.inner); }
    }

    /// Gets the inner type reference.
    pub fn inner(&self) -> sys::TypeRef { self.inner }
}
