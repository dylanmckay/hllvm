#pragma once

#include <llvm/IR/Value.h>

typedef void *ValueRef;

/// Unwraps a value to an LLVM value.
llvm::Value *unwrap(ValueRef Val) {
  return (llvm::Value*) Val;
}

extern "C" {
  void LLVMRustIRValueDump(const ValueRef);
}
