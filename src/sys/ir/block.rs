use {ContextRef, ValueRef};

cpp! {
    #include "support.h"

    #include "llvm/IR/Module.h"

    pub fn LLVMRustBasicBlockCreate(context: ContextRef as "llvm::LLVMContext*")
        -> ValueRef as "llvm::Value*" {
        return llvm::BasicBlock::Create(*context);
    }
}
