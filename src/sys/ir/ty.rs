use {TypeRef, ContextRef};

use libc;

cpp! {
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
                                   param_types: *mut TypeRef as "llvm::Type**",
                                   param_count: libc::c_uint as "unsigned",
                                   is_var_arg: bool as "bool")
        -> TypeRef as "llvm::Type*" {
        auto params = llvm::ArrayRef<llvm::Type*>(param_types, param_count);
        return llvm::FunctionType::get(result, params, is_var_arg);
    }

    pub fn LLVMRustStructTypeGet(context: ContextRef as "llvm::LLVMContext*",
                                 elements: *mut TypeRef as "llvm::Type**",
                                 element_count: libc::c_uint as "unsigned",
                                 is_packed: bool as "bool")
        -> TypeRef as "llvm::Type*" {
        llvm::ArrayRef<llvm::Type*> elems(elements, element_count);
        return llvm::StructType::get(*context, elems, is_packed);
    }
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

    // #[test]
    // fn can_get_struct_type() {
    //     let ctx = Context::new();
    //
    //     let i33 = unsafe { LLVMRustIntegerTypeGet(ctx.inner, 33) };
    //     let elems = Array::from_slice(&mut [i33]);
    //
    //     unsafe {
    //         LLVMRustStructTypeGet(ctx.inner, elems, false);
    //     }
    // }
}
