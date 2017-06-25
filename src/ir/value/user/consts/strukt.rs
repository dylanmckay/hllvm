use SafeWrapper;
use ir::{Constant, StructType, User};
use sys;

/// A constant structure value.
pub struct ConstantStruct<'ctx>(Constant<'ctx>);
impl_subtype!(ConstantStruct => Constant);

impl<'ctx> ConstantStruct<'ctx>
{
    /// Creates a new constant structure.
    pub fn new(ty: &StructType,
               elements: &[&Constant]) -> Self {
        let elements: Vec<_> = elements.iter().map(|e| e.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustConstantStructGet(ty.inner(), &elements);
            wrap_value!(inner => User => Constant => ConstantStruct)
        }
    }
}
