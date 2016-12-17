#pragma once

//! Core C++ include file.

#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Type.h>
#include <llvm/IR/Value.h>

#define DEFINE_PTR_CONVERSION_FUNCTIONS(DOWNSTREAM, UPSTREAM) \
inline UPSTREAM *unwrap(DOWNSTREAM Val) { return (UPSTREAM*) Val; } \
inline DOWNSTREAM wrap(UPSTREAM *Ctx) { return (DOWNSTREAM)Ctx; }

DEFINE_PTR_CONVERSION_FUNCTIONS(LLVMContextRef, llvm::LLVMContext);
DEFINE_PTR_CONVERSION_FUNCTIONS(LLVMTypeRef, llvm::Type);
DEFINE_PTR_CONVERSION_FUNCTIONS(LLVMValueRef, llvm::Value);
