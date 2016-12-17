#include "../llvm.h"

extern "C" {
  llvm::LLVMContext *LLVMRustCreateContext() {
    return new llvm::LLVMContext();
  }

  void LLVMRustDestroyContext(llvm::LLVMContext *Ctx) {
    delete Ctx;
  }
}
