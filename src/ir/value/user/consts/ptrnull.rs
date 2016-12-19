use SafeWrapper;
use ir::{Constant, Value, PointerType};
use sys;

/// A constant null pointer.
pub struct ConstantPointerNull<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantPointerNull => Constant);

impl<'ctx> ConstantPointerNull<'ctx>
{
    /// Creates a constant null pointer.
    pub fn new(ty: &PointerType) -> Self {
        unsafe {
            let inner = sys::LLVMRustConstantPointerNullGet(ty.inner());
            ConstantPointerNull(Constant(Value::from_inner(inner)))
        }
    }
}
