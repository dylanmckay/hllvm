/// The `struct` type.

use SafeWrapper;
use ir::{Context, Type, CompositeType};
use sys;

/// A type representing a structure.
pub struct StructType<'ctx>(CompositeType<'ctx>);

impl<'ctx> StructType<'ctx>
{
    pub fn new(elements: &[&Type],
               is_packed: bool,
               context: &Context) -> Self {
        let elements: Vec<_> = elements.iter().map(|t| t.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustStructTypeGet(context.inner(), &elements, is_packed);
            wrap_type!(inner => CompositeType => StructType)
        }
    }
}

impl_subtype!(StructType => CompositeType);

