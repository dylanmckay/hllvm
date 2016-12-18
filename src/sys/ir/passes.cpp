#include "../support.h"

#include <llvm/IR/LegacyPassManager.h>

extern "C" {
  llvm::legacy::PassManager *LLVMRustCreateLegacyPassManager() { return new llvm::legacy::PassManager(); }
  void LLVMRustDestroyLegacyPassManager(llvm::legacy::PassManager *PM) { delete PM; }
}
