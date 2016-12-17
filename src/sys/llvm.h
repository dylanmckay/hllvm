#pragma once

#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Value.h>

typedef void *ValueRef;

llvm::LLVMContext *unwrap(LLVMContextRef Val) { return (llvm::LLVMContext*) Val; }
llvm::Value *unwrap(ValueRef Val) { return (llvm::Value*) Val; }
