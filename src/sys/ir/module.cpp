#include "../llvm.h"

extern "C" {
  llvm::Module *LLVMRustCreateModule(const char *ModuleID,
                                     llvm::LLVMContext *Ctx) {
    return new llvm::Module(llvm::StringRef(ModuleID), *Ctx);
  }
}
