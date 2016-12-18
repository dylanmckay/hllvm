#include "../support.h"

#include <llvm/IR/LegacyPassManager.h>

extern "C" {
  llvm::legacy::PassManager *LLVMRustCreateLegacyPassManager() { return new llvm::legacy::PassManager(); }
  void LLVMRustDestroyLegacyPassManager(llvm::legacy::PassManager *PM) { delete PM; }

  bool LLVMRustLegacyPassManagerRun(llvm::legacy::PassManager *PM,
                                    llvm::Module *M) {
    return PM->run(*M);
  }
}
