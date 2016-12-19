use ir::{Context, Type, CompositeType};
use sys;

/// A type representing a structure.
pub struct StructType<'ctx>(CompositeType<'ctx>);

impl<'ctx> StructType<'ctx>
{
    pub fn new(elements: &[&Type],
               is_packed: bool,
               context: &Context) -> Self {
        let mut elements: Vec<_> = elements.iter().map(|t| t.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustStructTypeGet(context.inner(),
                elements.as_mut_ptr(), elements.len() as _, is_packed);
            StructType(CompositeType(Type::new(inner)))
        }
    }
}

impl_upcast!(StructType => CompositeType);
