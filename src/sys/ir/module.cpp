#include "../llvm.h"

extern "C" {
  LLVMModuleRef LLVMRustCreateModule(const char *ModuleID,
                                     LLVMContextRef Ctx) {
    return wrap(new llvm::Module(llvm::StringRef(ModuleID),
                                 *unwrap(Ctx)));
  }
}
