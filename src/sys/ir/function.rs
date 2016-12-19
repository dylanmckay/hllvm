use {ValueRef, TypeRef, ModuleRef};
use libc;

cpp! {
    #include "support.h"

    #include "llvm/IR/Module.h"

    pub fn LLVMRustFunctionCreate(ty: TypeRef as "llvm::Type*",
                                  linkage: libc::c_uint as "unsigned",
                                  name: *const libc::c_char as "const char*",
                                  module: ModuleRef as "llvm::Module*")
        -> ValueRef as "llvm::Value*" {
        return llvm::Function::Create(support::cast<llvm::FunctionType>(ty),
            (llvm::GlobalValue::LinkageTypes)linkage, name, module);
    }
}
