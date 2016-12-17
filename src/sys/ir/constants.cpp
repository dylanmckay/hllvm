#include "../llvm.h"

#include <llvm/IR/Constants.h>

extern "C" {
  llvm::Value *LLVMRustConstantIntGetTrue(llvm::LLVMContext *Ctx) {
    return llvm::ConstantInt::getTrue(*Ctx);
  }

  llvm::Value *LLVMRustConstantIntGetFalse(llvm::LLVMContext *Ctx) {
    return llvm::ConstantInt::getFalse(*Ctx);
  }
}
