use {ValueRef, ContextRef, TypeRef, Linkage, ThreadLocalMode};
use libc;

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

    pub fn LLVMRustConstantAggregateZeroGet(ty: TypeRef as "llvm::Type*")
        -> ValueRef as "llvm::Value*" {
        return llvm::ConstantAggregateZero::get(ty);
    }

    pub fn LLVMRustConstantFPGetDouble(ty: TypeRef as "llvm::Type*",
                                       value: f64 as "double")
        -> ValueRef as "llvm::Value*" {
        return llvm::ConstantFP::get(ty, value);
    }

    pub fn LLVMRustConstantFPGetString(ty: TypeRef as "llvm::Type*",
                                       value: *const libc::c_char as "const char*")
        -> ValueRef as "llvm::Value*" {
        return llvm::ConstantFP::get(ty, value);
    }

    pub fn LLVMRustConstantPointerNullGet(ty: TypeRef as "llvm::Type*")
        -> ValueRef as "llvm::Value*" {
        return llvm::ConstantPointerNull::get(
            support::cast<llvm::PointerType>(ty));
    }

    pub fn LLVMRustConstantStructGet(ty: TypeRef as "llvm::Type*",
                                     elements: &[ValueRef] as "support::Slice<llvm::Value*>")
        -> ValueRef as "llvm::Value*" {
        support::Slice<llvm::Constant*> elems = elements.cast<llvm::Constant*>();

        return llvm::ConstantStruct::get(
            support::cast<llvm::StructType>(ty), elems.ref());
    }

    pub fn LLVMRustConstantVectorGet(values: &[ValueRef] as "support::Slice<llvm::Value*>")
        -> ValueRef as "llvm::Value*" {
        support::Slice<llvm::Constant*> vals = values.cast<llvm::Constant*>();
        return llvm::ConstantVector::get(vals.ref());
    }

    pub fn LLVMRustUndefValueGet(ty: TypeRef as "llvm::Type*")
        -> ValueRef as "llvm::Value*" {
        return llvm::UndefValue::get(ty);
    }

    pub fn LLVMRustCreateGlobalVariable(ty: TypeRef as "llvm::Type*",
                                        is_constant: bool as "bool",
                                        linkage: Linkage as "unsigned",
                                        initializer: ValueRef as "llvm::Value*",
                                        tls_mode: ThreadLocalMode as "unsigned",
                                        address_space: libc::c_uint as "unsigned",
                                        is_externally_initialized: bool as "bool")
        -> ValueRef as "llvm::Value*" {
        return new llvm::GlobalVariable(ty, is_constant, (llvm::GlobalValue::LinkageTypes)linkage,
            support::cast<llvm::Constant>(initializer), "", (llvm::GlobalValue::ThreadLocalMode)tls_mode, address_space,
            is_externally_initialized);
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
