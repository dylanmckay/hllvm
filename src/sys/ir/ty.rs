use {Array, TypeRef, ContextRef};

use libc;

extern "C" {
    pub fn LLVMRustTypeGetVoidTy(ctx: ContextRef) -> TypeRef;
    pub fn LLVMRustTypeDump(ty: TypeRef);
    pub fn LLVMRustIntegerTypeGet(ctx: ContextRef, NumBits: libc::c_int) -> TypeRef;

    pub fn LLVMRustFunctionTypeGet(result: TypeRef,
                                   param_types: *mut TypeRef,
                                   param_count: libc::c_uint,
                                   is_var_arg: bool) -> TypeRef;

    pub fn LLVMRustStructTypeGet(ctx: ContextRef,
                                 elements: Array<TypeRef>,
                                 is_packed: bool) -> TypeRef;
}

#[cfg(test)]
mod test {
    use super::*;
    use test_support::Context;
    use std::ptr;
    use Array;

    #[test]
    fn can_dump_void_type() {
        let ctx = Context::new();

        unsafe {
            let ty = LLVMRustTypeGetVoidTy(ctx.inner);
            ::LLVMRustTypeDump(ty);
        }
    }

    #[test]
    fn can_get_u22_type() {
        let ctx = Context::new();

        unsafe { LLVMRustIntegerTypeGet(ctx.inner, 22) };
    }

    #[test]
    fn can_create_void_function_type() {
        let ctx = Context::new();

        unsafe {
            let void = LLVMRustTypeGetVoidTy(ctx.inner);
            LLVMRustFunctionTypeGet(void, ptr::null_mut(), 0, false);
        }
    }

    #[test]
    fn can_get_struct_type() {
        let ctx = Context::new();

        let i33 = unsafe { LLVMRustIntegerTypeGet(ctx.inner, 33) };
        let elems = Array::from_slice(&mut [i33]);

        unsafe {
            LLVMRustStructTypeGet(ctx.inner, elems, false);
        }
    }
}
