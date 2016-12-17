#include "../llvm.h"

extern "C" {
  LLVMTypeRef LLVMRustTypeGetVoidTy(LLVMContextRef Ctx) {
    return wrap(llvm::Type::getVoidTy(*unwrap(Ctx)));
  }

  void LLVMRustTypeDump(LLVMTypeRef Ty) {
    unwrap(Ty)->dump();
  }

  LLVMTypeRef LLVMRustIntegerTypeGet(LLVMContextRef Ctx, unsigned NumBits) {
    return wrap(llvm::IntegerType::get(*unwrap(Ctx), NumBits));
  }
}
