#include "../llvm.h"

#include <llvm/IR/Instructions.h>

extern "C" {
  llvm::Value *LLVMRustCreateReturnInst(llvm::LLVMContext *Ctx,
                                        llvm::Value *RetVal) {
    return llvm::ReturnInst::Create(*Ctx, RetVal);
  }
}
