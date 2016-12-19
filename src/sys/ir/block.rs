use {ContextRef, ValueRef};
use libc;

cpp! {
    #include "support.h"

    #include "llvm/IR/Module.h"

    pub fn LLVMRustBasicBlockCreate(context: ContextRef as "llvm::LLVMContext*",
                                    name: *const libc::c_char as "const char*",
                                    func: ValueRef as "llvm::Value*",
                                    insert_before: ValueRef as "llvm::Value*") -> ValueRef as "llvm::Value*" {
        return llvm::BasicBlock::Create(*context, name,
            support::cast<llvm::Function>(func), support::cast<llvm::BasicBlock>(insert_before));
    }
}
