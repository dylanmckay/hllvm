#include "../llvm.h"

#include <llvm/IR/Constants.h>

extern "C" {
  LLVMValueRef LLVMRustConstantIntGetTrue(LLVMContextRef Ctx) {
    return wrap(llvm::ConstantInt::getTrue(*unwrap(Ctx)));
  }

  LLVMValueRef LLVMRustConstantIntGetFalse(LLVMContextRef Ctx) {
    return wrap(llvm::ConstantInt::getFalse(*unwrap(Ctx)));
  }
}
