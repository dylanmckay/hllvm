#include "../support.h"

#include <llvm/IR/Module.h>

extern "C" {
  llvm::Function *LLVMRustFunctionCreate(llvm::Type *Ty,
                                         unsigned Linkage,
                                         const char *Name,
                                         llvm::Module *Mod) {
    llvm::FunctionType *FnTy = llvm::dyn_cast<llvm::FunctionType>(Ty);
    assert(FnTy && "type is not a function");

    return llvm::Function::Create(FnTy, (llvm::GlobalValue::LinkageTypes)Linkage, Name, Mod);
  }
}
