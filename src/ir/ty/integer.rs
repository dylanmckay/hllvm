use ir::{Type, Context};
use sys;

/// A type representing a function.
pub struct IntegerType<'ctx>(Type<'ctx>);

impl<'ctx> IntegerType<'ctx>
{
    /// Gets an integer type.
    pub fn integer(num_bits: usize,
                   context: &Context) -> Self {
        let inner = unsafe { sys::LLVMRustIntegerTypeGet(context.inner(),
                                                         num_bits as _) };
        IntegerType(Type::new(inner))
    }
}

impl_upcast!(IntegerType => Type);
