use {ValueRef, TypeRef};

extern "C" {
    pub fn LLVMRustValueDump(_: ValueRef);
    pub fn LLVMRustValueGetType(_: ValueRef) -> TypeRef;
}

#[cfg(test)]
mod test {
    use test_support::Context;

    #[test]
    fn can_get_type_from_constant() {
        let ctx = Context::new();

        unsafe {
            let val = ::LLVMRustConstantIntGetTrue(ctx.inner);
            ::LLVMRustValueGetType(val);
        }
    }
}
