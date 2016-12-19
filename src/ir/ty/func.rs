use SafeWrapper;
use ir::Type;
use sys;

/// A type representing a function.
pub struct FunctionType<'ctx>(Type<'ctx>);

impl<'ctx> FunctionType<'ctx>
{
    /// Gets a function type.
    pub fn new(result: &Type<'ctx>,
               params: &[Type<'ctx>],
               is_var_arg: bool) -> Self {
        let mut params: Vec<sys::TypeRef> = params.iter().map(Type::inner).collect();

        unsafe {
            let inner = sys::LLVMRustFunctionTypeGet(result.inner(),
                params.as_mut_ptr(), params.len() as _, is_var_arg);
            FunctionType(Type::from_inner(inner))
        }
    }
}

impl_upcast!(FunctionType => Type);
