#include "../support.h"

#include <llvm/IR/Instructions.h>

extern "C" {
  void LLVMRustInstructionInsertAfter(llvm::Value *Inst,
                                      llvm::Value *InstAfter) {
    llvm::Instruction *I1 = llvm::dyn_cast<llvm::Instruction>(Inst);
    llvm::Instruction *I2 = llvm::dyn_cast<llvm::Instruction>(InstAfter);
    assert(I1 && "not a instruction");
    assert(I2 && "not a instruction");

    I1->insertAfter(I2);
  }

  void LLVMRustInstructionAppend(llvm::Value *Inst, llvm::Value *BB) {
    llvm::BasicBlock *Block = llvm::dyn_cast<llvm::BasicBlock>(BB);
    llvm::Instruction *I = llvm::dyn_cast<llvm::Instruction>(Inst);
    assert(Block && "not a basic block");
    assert(I && "not a instruction");

    Block->getInstList().push_back(I);
  }

  llvm::Value *LLVMRustCreateReturnInst(llvm::LLVMContext *Ctx,
                                        llvm::Value *RetVal) {
    return llvm::ReturnInst::Create(*Ctx, RetVal);
  }
}
