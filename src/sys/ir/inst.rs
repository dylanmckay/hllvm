use {ContextRef, ValueRef};

cpp! {
    #include "llvm/IR/Instructions.h"

    pub fn LLVMRustInstructionInsertAfter(inst: ValueRef as "llvm::Value*",
                                          after: ValueRef as "llvm::Value*") {
        llvm::Instruction *i1 = llvm::dyn_cast<llvm::Instruction>(inst);
        llvm::Instruction *i2 = llvm::dyn_cast<llvm::Instruction>(after);
        assert(i1 && "not a instruction");
        assert(i2 && "not a instruction");

        i1->insertAfter(i2);
    }

    pub fn LLVMRustInstructionAppend(inst_val: ValueRef as "llvm::Value*",
                                     block_val: ValueRef as "llvm::Value*") {
        llvm::BasicBlock *block = llvm::dyn_cast<llvm::BasicBlock>(block_val);
        llvm::Instruction *inst = llvm::dyn_cast<llvm::Instruction>(inst_val);
        assert(block && "not a basic block");
        assert(inst && "not a instruction");

        block->getInstList().push_back(inst);
    }

    pub fn LLVMRustCreateReturnInst(context: ContextRef as "llvm::LLVMContext*",
                                    ret_val: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::ReturnInst::Create(*context, ret_val);
    }
}
