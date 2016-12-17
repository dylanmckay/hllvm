#include "../llvm.h"

extern "C" {
  void LLVMRustIRValueDump(const ValueRef Val) {
    unwrap(Val)->dump();
  }
}
