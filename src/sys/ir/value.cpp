#include "../llvm.h"

extern "C" {
  void LLVMRustValueDump(const LLVMValueRef Val) {
    unwrap(Val)->dump();
  }
}
