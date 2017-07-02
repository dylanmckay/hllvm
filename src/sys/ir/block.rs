use {ContextRef, ValueRef};

cpp! {
    #include "ffi_helpers.h"

    #include "llvm/IR/Module.h"

    pub fn LLVMRustBasicBlockCreate(context: ContextRef as "llvm::LLVMContext*")
        -> ValueRef as "llvm::Value*" {
        return llvm::BasicBlock::Create(*context);
    }
}
