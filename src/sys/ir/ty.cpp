#include "../llvm.h"

extern "C" {
  llvm::Type *LLVMRustTypeGetVoidTy(llvm::LLVMContext *Ctx) {
    return llvm::Type::getVoidTy(*Ctx);
  }

  void LLVMRustTypeDump(llvm::Type *Ty) {
    Ty->dump();
  }

  llvm::Type *LLVMRustIntegerTypeGet(llvm::LLVMContext *Ctx, unsigned NumBits) {
    return llvm::IntegerType::get(*Ctx, NumBits);
  }

  llvm::Type *LLVMRustFunctionTypeGet(llvm::Type *Result,
                                      llvm::Type **ParamTypes,
                                      unsigned ParamCount,
                                      bool IsVarArg) {
    auto Params = llvm::ArrayRef<llvm::Type*>(ParamTypes, ParamCount);
    return llvm::FunctionType::get(Result, Params, IsVarArg);
  }
}
