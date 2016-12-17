#include "../llvm.h"

extern "C" {
  llvm::BasicBlock *LLVMRustBasicBlockCreate(llvm::LLVMContext *Ctx,
                                             const char *Name,
                                             llvm::Function *Parent,
                                             llvm::BasicBlock *InsertBefore) {
    return llvm::BasicBlock::Create(*Ctx, Name, Parent, InsertBefore);
  }
}
