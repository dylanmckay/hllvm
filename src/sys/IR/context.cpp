#include "../llvm.h"

extern "C" {
  LLVMContextRef LLVMRustCreateContext() {
    return wrap(new llvm::LLVMContext());
  }

  void LLVMRustDestroyContext(LLVMContextRef ctx) {
    delete unwrap(ctx);
  }
}
