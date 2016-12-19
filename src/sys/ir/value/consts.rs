use {ValueRef, ContextRef, TypeRef};

cpp! {
    #include "support.h"

    #include "llvm/IR/Constants.h"
    #include "llvm/IR/LLVMContext.h"

    pub fn LLVMRustConstantIntGetTrue(context: ContextRef as "llvm::LLVMContext*")
        -> ValueRef as "llvm::Value*" {
        return llvm::ConstantInt::getTrue(*context);
    }

    pub fn LLVMRustConstantIntGetFalse(context: ContextRef as "llvm::LLVMContext*")
        -> ValueRef as "llvm::Value*" {
        return llvm::ConstantInt::getFalse(*context);
    }

    pub fn LLVMRustConstantIntGetSigned(ty: TypeRef as "llvm::Type*",
                                        val: i64 as "int64_t")
        -> ValueRef as "llvm::Value*" {
        return llvm::ConstantInt::getSigned(ty, val);
    }

    pub fn LLVMRustConstantIntGetUnsigned(ty: TypeRef as "llvm::Type*",
                                          val: u64 as "uint64_t")
        -> ValueRef as "llvm::Value*" {
        return llvm::ConstantInt::get(ty, val);
    }

    pub fn LLVMRustBlockAddressGet(block: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::BlockAddress::get(support::cast<llvm::BasicBlock>(block));
    }
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
