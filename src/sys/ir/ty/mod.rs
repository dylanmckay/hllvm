use {TypeRef, ContextRef};

use libc;

cpp! {
    #include "ffi_helpers.h"

    #include "llvm/IR/Module.h"
    #include "llvm/IR/LLVMContext.h"

    pub fn LLVMRustTypeGetVoidTy(context: ContextRef as "llvm::LLVMContext*")
        -> TypeRef as "llvm::Type*" {
        return llvm::Type::getVoidTy(*context);
    }

    pub fn LLVMRustTypeDump(ty: TypeRef as "llvm::Type*") {
        ty->dump();
    }

    pub fn LLVMRustIntegerTypeGet(context: ContextRef as "llvm::LLVMContext*",
                                  num_bits: libc::c_int as "int")
        -> TypeRef as "llvm::Type*" {
        return llvm::IntegerType::get(*context, num_bits);
    }

    pub fn LLVMRustFunctionTypeGet(result: TypeRef as "llvm::Type*",
                                   params: &[TypeRef] as "support::Slice<llvm::Type*>",
                                   is_var_arg: bool as "bool")
        -> TypeRef as "llvm::Type*" {
        return llvm::FunctionType::get(result, params.ref(), is_var_arg);
    }

    pub fn LLVMRustStructTypeGet(context: ContextRef as "llvm::LLVMContext*",
                                 elements: &[TypeRef] as "support::Slice<llvm::Type*>",
                                 is_packed: bool as "bool")
        -> TypeRef as "llvm::Type*" {
        return llvm::StructType::get(*context, elements.ref(), is_packed);
    }

    pub fn LLVMRustArrayTypeGet(element_type: TypeRef as "llvm::Type*",
                                num_elements: u64 as "uint64_t")
        -> TypeRef as "llvm::Type*" {
        return llvm::ArrayType::get(element_type, num_elements);
    }

    pub fn LLVMRustVectorTypeGet(element_type: TypeRef as "llvm::Type*",
                                 num_elements: u64 as "uint64_t")
        -> TypeRef as "llvm::Type*" {
        return llvm::VectorType::get(element_type, num_elements);
    }

    pub fn LLVMRustPointerTypeGet(element_type: TypeRef as "llvm::Type*",
                                  address_space: libc::c_uint as "unsigned")
        -> TypeRef as "llvm::Type*" {
        return llvm::PointerType::get(element_type, address_space);
    }
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

    #[test]
    fn can_create_void_function_type() {
        let ctx = Context::new();

        unsafe {
            let void = LLVMRustTypeGetVoidTy(ctx.inner);
            LLVMRustFunctionTypeGet(void, &[], false);
        }
    }

    #[test]
    fn can_get_struct_type() {
        let ctx = Context::new();

        let i33 = unsafe { LLVMRustIntegerTypeGet(ctx.inner, 33) };

        unsafe {
            LLVMRustStructTypeGet(ctx.inner, &[i33], false);
        }
    }
}
