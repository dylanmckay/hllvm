use {ValueRef, TypeRef, ModuleRef};
use libc;

cpp! {
    #include "llvm/IR/Module.h"

    pub fn LLVMRustFunctionCreate(ty: TypeRef as "llvm::Type*",
                                  linkage: libc::c_uint as "unsigned",
                                  name: *const libc::c_char as "const char*",
                                  module: ModuleRef as "llvm::Module*")
        -> ValueRef as "llvm::Value*" {
        llvm::FunctionType *fn_ty = llvm::dyn_cast<llvm::FunctionType>(ty);
        assert(fn_ty && "type is not a function");

        return llvm::Function::Create(fn_ty, (llvm::GlobalValue::LinkageTypes)linkage, name, module);
    }
}
