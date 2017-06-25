use SafeWrapper;
use ir::{SequentialType, CompositeType, Type};
use sys;

/// An array type.
pub struct ArrayType<'ctx>(SequentialType<'ctx>);

impl<'ctx> ArrayType<'ctx>
{
    /// Creates a new array type.
    pub fn new(element_type: &Type, num_elements: u64) -> Self {
        unsafe {
            let inner = sys::LLVMRustArrayTypeGet(element_type.inner(), num_elements);
            wrap_type!(inner => CompositeType => SequentialType => ArrayType)
        }
    }
}

impl_subtype!(ArrayType => SequentialType);
