#include "../llvm.h"

extern "C" {
  void LLVMRustValueDump(LLVMValueRef Val) {
    unwrap(Val)->dump();
  }

  LLVMTypeRef LLVMRustValueGetType(LLVMValueRef Val) {
    return wrap(unwrap(Val)->getType());
  }
}
