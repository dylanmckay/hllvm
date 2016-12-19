use {ContextRef, ValueRef};
use libc;

cpp! {
    #include "llvm/IR/Module.h"

    pub fn LLVMRustBasicBlockCreate(context: ContextRef as "llvm::LLVMContext*",
                                    name: *const libc::c_char as "const char*",
                                    func_val: ValueRef as "llvm::Value*",
                                    insert_before_val: ValueRef as "llvm::Value*") -> ValueRef as "llvm::Value*" {
        llvm::Function *func = (func_val == nullptr) ? nullptr : llvm::dyn_cast<llvm::Function>(func_val);
        llvm::BasicBlock *insert_before = (insert_before_val == nullptr) ? nullptr : llvm::dyn_cast<llvm::BasicBlock>(insert_before_val);
        assert(func && "not a function");

        return llvm::BasicBlock::Create(*context, name, func, insert_before);
    }
}
