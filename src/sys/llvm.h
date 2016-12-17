#pragma once

//! Core C++ include file.

#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Type.h>
#include <llvm/IR/Value.h>

typedef void *ValueRef;

inline llvm::LLVMContext *unwrap(LLVMContextRef Val) { return (llvm::LLVMContext*) Val; }
inline llvm::Value *unwrap(ValueRef Val) { return (llvm::Value*) Val; }

inline LLVMTypeRef wrap(llvm::Type *Ty) { return (LLVMTypeRef)Ty; }
inline LLVMContextRef wrap(llvm::LLVMContext *Ctx) { return (LLVMContextRef)Ctx; }
