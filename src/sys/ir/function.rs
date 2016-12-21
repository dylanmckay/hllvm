use {Linkage, ValueRef, TypeRef, ModuleRef};
use libc;

cpp! {
    #include "support.h"

    #include "llvm/IR/Module.h"

    pub fn LLVMRustFunctionCreate(ty: TypeRef as "llvm::Type*",
                                  linkage: Linkage as "unsigned",
                                  name: *const libc::c_char as "const char*",
                                  module: ModuleRef as "llvm::Module*")
        -> ValueRef as "llvm::Value*" {
        return llvm::Function::Create(support::cast<llvm::FunctionType>(ty),
            (llvm::GlobalValue::LinkageTypes)linkage, name, module);
    }

    pub fn LLVMRustFunctionAddBlock(func: ValueRef as "llvm::Value*",
                                    block: ValueRef as "llvm::Value*") {
        support::cast<llvm::Function>(func)->getBasicBlockList().addNodeToList(
            support::cast<llvm::BasicBlock>(block));
    }
}
