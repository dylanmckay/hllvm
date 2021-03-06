//! Integer type.

use SafeWrapper;
use ir::{Type, Context};
use sys;

/// A type representing a function.
pub struct IntegerType<'ctx>(Type<'ctx>);

impl<'ctx> IntegerType<'ctx>
{
    /// Gets an integer type.
    pub fn new(num_bits: usize,
               context: &Context) -> Self {
        unsafe {
            let inner = sys::LLVMRustIntegerTypeGet(context.inner(),
                                                    num_bits as _);
            IntegerType(Type::from_inner(inner))
        }
    }
}

impl_subtype!(IntegerType => Type);

