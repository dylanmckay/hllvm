use {TypeRef, ContextRef};

use libc;

extern "C" {
    pub fn LLVMRustTypeGetVoidTy(ctx: ContextRef) -> TypeRef;
    pub fn LLVMRustTypeDump(ty: TypeRef);
    pub fn LLVMRustIntegerTypeGet(ctx: ContextRef, NumBits: libc::c_int) -> TypeRef;
}

#[cfg(test)]
mod test {
    use super::*;
    use test_support::Context;

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
}
