use ir;

pub enum OpaqueType { }

type TypeRef = *mut OpaqueType;

extern "C" {
    pub fn LLVMRustTypeGetVoidTy(ctx: ir::ContextRef) -> TypeRef;
    pub fn LLVMRustTypeDump(ty: TypeRef);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_dump_void_type() {
        // let ty = unsafe { LLVMRustTypeGetVoidTy() };
    }
}
