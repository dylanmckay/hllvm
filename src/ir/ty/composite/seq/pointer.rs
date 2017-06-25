use SafeWrapper;
use ir::{SequentialType, CompositeType, Type};
use sys;

/// A pointer.
pub struct PointerType<'ctx>(SequentialType<'ctx>);

impl<'ctx> PointerType<'ctx>
{
    /// Creates a new pointer type.
    pub fn new(element_ty: &Type,
               address_space: u32) -> Self {
        unsafe {
            let inner = sys::LLVMRustPointerTypeGet(element_ty.inner(), address_space as _);
            wrap_type!(inner => CompositeType => SequentialType => PointerType)
        }
    }
}

impl_subtype!(PointerType => SequentialType);
