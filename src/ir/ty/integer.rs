use ir::{Type, Context};
use sys;

/// A type representing a function.
pub struct IntegerType<'ctx>
{
    ty: Type<'ctx>,
}

impl<'ctx> IntegerType<'ctx>
{
    /// Gets an integer type.
    pub fn integer(num_bits: usize,
                   context: &Context) -> Self {
        let inner = unsafe { sys::LLVMRustIntegerTypeGet(context.inner(),
                                                         num_bits as _) };
        IntegerType { ty: Type::new(inner) }
    }

    pub fn upcast_ref(&self) -> &Type<'ctx> { &self.ty }
    pub fn upcast(self) -> Type<'ctx> { self.ty }
}

impl<'a> AsRef<Type<'a>> for IntegerType<'a>
{
    fn as_ref(&self) -> &Type<'a> { &self.ty }
}
