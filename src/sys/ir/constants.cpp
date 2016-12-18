#include "../support.h"

#include <llvm/IR/Constants.h>

extern "C" {
  llvm::Value *LLVMRustConstantIntGetTrue(llvm::LLVMContext *Ctx) {
    return llvm::ConstantInt::getTrue(*Ctx);
  }

  llvm::Value *LLVMRustConstantIntGetFalse(llvm::LLVMContext *Ctx) {
    return llvm::ConstantInt::getFalse(*Ctx);
  }

  llvm::Value *LLVMRustConstantIntGetSigned(llvm::Type *Ty, int64_t Val) {
    return llvm::ConstantInt::getSigned(Ty, Val);
  }

  llvm::Value *LLVMRustConstantIntGetUnsigned(llvm::Type *Ty, uint64_t Val) {
    return llvm::ConstantInt::get(Ty, Val);
  }
}
