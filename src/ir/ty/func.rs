//! The `Function` type.

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
        let params: Vec<sys::TypeRef> = params.iter().map(Type::inner).collect();

        unsafe {
            let inner = sys::LLVMRustFunctionTypeGet(result.inner(), &params, is_var_arg);
            FunctionType(Type::from_inner(inner))
        }
    }
}

impl_subtype!(FunctionType => Type);

