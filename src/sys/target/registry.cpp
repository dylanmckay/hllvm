#include "../llvm.h"

#include <llvm/Support/TargetRegistry.h>
#include <llvm/Support/TargetSelect.h>

extern "C" {
  void LLVMRustInitializeAllTargets() {
    llvm::InitializeAllTargetInfos();
    llvm::InitializeAllTargets();
    llvm::InitializeAllTargetMCs();
    llvm::InitializeAllAsmParsers();
    llvm::InitializeAllAsmPrinters();
  }

  // Loads the list of targets into an array.
  // Returns the number of targets loaded.
  size_t LLVMRustTargetRegistryTargets(llvm::Target **Dest,
                                       size_t MaxCount) {
    int CurCount = 0;

    for (const llvm::Target &Target : llvm::TargetRegistry::targets()) {
      assert(CurCount <= MaxCount && "too many targets to fit in the list");
      Dest[CurCount] = const_cast<llvm::Target*>(&Target);

      CurCount += 1;
    }

    return CurCount;
  }
}
