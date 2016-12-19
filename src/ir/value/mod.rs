pub use self::block::Block;
pub use self::argument::Argument;
pub use self::inlineasm::InlineAsm;
pub use self::user::*;

pub mod block;
pub mod argument;
pub mod inlineasm;
pub mod user;

use SafeWrapper;
use ir::Type;
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
    /// Get the type of the value.
    pub fn ty(&self) -> Type {
        unsafe { Type::from_inner(sys::LLVMRustValueGetType(self.inner)) }
    }

    /// Dumps the value to standard error.
    pub fn dump(&self) {
        unsafe {
            sys::LLVMRustValueDump(self.inner);
        }
    }
}

impl<'ctx> SafeWrapper for Value<'ctx>
{
    type Inner = sys::ValueRef;

    unsafe fn from_inner(inner: sys::ValueRef) -> Self {
        Value { inner: inner, phantom: marker::PhantomData }
    }

    fn inner(&self) -> sys::ValueRef { self.inner }
}

#[cfg(test)]
mod test {
    use ir;

    #[test]
    fn can_get_ty_of_true() {
        let context = ir::Context::new();

        ir::ConstantInt::boolean_true(&context).ty();
    }
}
