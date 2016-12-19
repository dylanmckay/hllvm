pub use self::func::FunctionType;
pub use self::integer::IntegerType;
pub use self::composite::*;

pub mod func;
pub mod integer;
pub mod composite;

use SafeWrapper;
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
    /// Gets the `void` type.
    pub fn void(context: &Context) -> Self {
        unsafe { Type::from_inner(sys::LLVMRustTypeGetVoidTy(context.inner())) }
    }

    /// Dump the type to standard error.
    pub fn dump(&self) {
        unsafe { sys::LLVMRustTypeDump(self.inner); }
    }
}

impl<'ctx> SafeWrapper for Type<'ctx>
{
    type Inner = sys::TypeRef;

    unsafe fn from_inner(inner: sys::TypeRef) -> Self {
        Type { inner: inner, phantom: marker::PhantomData }
    }

    fn inner(&self) -> sys::TypeRef { self.inner }
}
