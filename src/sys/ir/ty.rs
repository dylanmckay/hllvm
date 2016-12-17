use ir;

pub enum OpaqueType { }

pub type TypeRef = *mut OpaqueType;

extern "C" {
    pub fn LLVMRustTypeGetVoidTy(ctx: ir::ContextRef) -> TypeRef;
    pub fn LLVMRustTypeDump(ty: TypeRef);
}

#[cfg(test)]
mod test {
    use super::*;
    use test_support::Context;

    #[test]
    fn can_dump_void_type() {
        let ctx = Context::new();

        unsafe {
            let ty = ::LLVMRustTypeGetVoidTy(ctx.inner);
            ::LLVMRustTypeDump(ty);
        }
    }
}
