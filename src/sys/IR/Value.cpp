#include "../llvm.h"

extern "C" {
  void LLVMRustIRValueDump(const LLVMValueRef Val) {
    unwrap(Val)->dump();
  }
}
