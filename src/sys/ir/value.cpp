#include "../llvm.h"

#include <llvm/IR/Value.h>

extern "C" {
  void LLVMRustValueDump(llvm::Value *Val) {
    Val->dump();
  }

  llvm::Type *LLVMRustValueGetType(llvm::Value *Val) {
    return Val->getType();
  }
}
