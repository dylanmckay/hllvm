use SafeWrapper;
use ir::{Constant, PointerType, User};
use sys;

/// A constant null pointer.
pub struct ConstantPointerNull<'ctx>(Constant<'ctx>);
impl_subtype!(ConstantPointerNull => Constant);

impl<'ctx> ConstantPointerNull<'ctx>
{
    /// Creates a constant null pointer.
    pub fn new(ty: &PointerType) -> Self {
        unsafe {
            let inner = sys::LLVMRustConstantPointerNullGet(ty.inner());
            wrap_value!(inner => User => Constant => ConstantPointerNull)
        }
    }
}
