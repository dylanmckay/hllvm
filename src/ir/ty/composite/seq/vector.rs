use SafeWrapper;
use ir::{SequentialType, CompositeType, Type};
use sys;

/// A vector type.
pub struct VectorType<'ctx>(SequentialType<'ctx>);

impl<'ctx> VectorType<'ctx>
{
    /// Creates a new vector type.
    pub fn new(element_type: &Type, num_elements: u64) -> Self {
        unsafe {
            let inner = sys::LLVMRustVectorTypeGet(element_type.inner(),
                num_elements);
            VectorType(SequentialType(CompositeType(Type::from_inner(inner))))
        }
    }
}

impl_upcast!(VectorType => SequentialType);
