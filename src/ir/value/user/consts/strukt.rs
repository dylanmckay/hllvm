use SafeWrapper;
use ir::{Value, Constant, StructType};
use sys;

/// A constant structure value.
pub struct ConstantStruct<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantStruct => Constant);

impl<'ctx> ConstantStruct<'ctx>
{
    /// Creates a new constant structure.
    pub fn new(ty: &StructType,
               elements: &[&Constant]) -> Self {
        let elements: Vec<_> = elements.iter().map(|e| e.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustConstantStructGet(ty.inner(), &elements);
            ConstantStruct(Constant(Value::from_inner(inner)))
        }
    }
}
