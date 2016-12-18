#include "../llvm.h"

#include <llvm/Support/TargetRegistry.h>

extern "C" {
  const char *LLVMRustTargetGetName(llvm::Target *Ty) {
    return Ty->getName();
  }

  const char *LLVMRustTargetGetShortDescription(llvm::Target *Ty) {
    return Ty->getShortDescription();
  }

  bool LLVMRustTargetHasMCAsmBackend(llvm::Target *Ty) {
    return Ty->hasMCAsmBackend();
  }

  bool LLVMRustTargetHasTargetMachine(llvm::Target *Ty) {
    return Ty->hasTargetMachine();
  }

  bool LLVMRustTargetHasJIT(llvm::Target *Ty) {
    return Ty->hasJIT();
  }
}
