use {ValueRef, ContextRef};

extern "C" {
  pub fn LLVMRustConstantIntGetTrue(_: ContextRef) -> ValueRef;
  pub fn LLVMRustConstantIntGetFalse(_: ContextRef) -> ValueRef;
}

#[cfg(test)]
mod test {
    use super::*;
    use test_support::Context;

    #[test]
    fn can_get_and_dump_constant_int_true() {
        unsafe {
            let context = ::LLVMRustCreateContext();
            let val = LLVMRustConstantIntGetTrue(context);
            ::LLVMRustValueDump(val);
        }
    }

    #[test]
    fn can_get_and_dump_constant_int_false() {
        unsafe {
            let context = ::LLVMRustCreateContext();
            let val = LLVMRustConstantIntGetFalse(context);
            ::LLVMRustValueDump(val);
        }
    }

    #[test]
    fn false_and_true_are_same_ty() {
        let ctx = Context::new();

        unsafe {
            let t = ::LLVMRustConstantIntGetTrue(ctx.inner);
            let f = ::LLVMRustConstantIntGetFalse(ctx.inner);

            assert_eq!(::LLVMRustValueGetType(t), ::LLVMRustValueGetType(f));
        }
    }
}
