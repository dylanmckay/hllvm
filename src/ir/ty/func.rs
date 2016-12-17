use ir::Type;
use sys;

/// A type representing a function.
pub struct FunctionType<'ctx>
{
    ty: Type<'ctx>,
}

impl<'ctx> FunctionType<'ctx>
{
    /// Gets a function type.
    pub fn new(result: &Type<'ctx>,
               params: &[Type<'ctx>],
               is_var_arg: bool) -> Self {
        let mut params: Vec<sys::TypeRef> = params.iter().map(Type::inner).collect();

        let inner = unsafe {
            sys::LLVMRustFunctionTypeGet(result.inner(),
                params.as_mut_ptr(), params.len() as _, is_var_arg)
        };

        FunctionType { ty: Type::new(inner) }
    }

    pub fn upcast_ref(&self) -> &Type<'ctx> { &self.ty }
    pub fn upcast(self) -> Type<'ctx> { self.ty }
}
