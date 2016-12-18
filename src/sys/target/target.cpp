#include "../support.h"

#include <llvm/IR/LegacyPassManager.h>
#include <llvm/Support/TargetRegistry.h>
#include <llvm/Target/TargetMachine.h>
#include <llvm/Target/TargetOptions.h>

extern "C" {
  const char *LLVMRustTargetGetName(llvm::Target *T) {
    return T->getName();
  }

  const char *LLVMRustTargetGetShortDescription(llvm::Target *T) {
    return T->getShortDescription();
  }

  bool LLVMRustTargetHasMCAsmBackend(llvm::Target *T) {
    return T->hasMCAsmBackend();
  }

  bool LLVMRustTargetHasTargetMachine(llvm::Target *T) {
    return T->hasTargetMachine();
  }

  bool LLVMRustTargetHasJIT(llvm::Target *T) {
    return T->hasJIT();
  }

  llvm::TargetMachine* LLVMRustTargetCreateTargetMachine(llvm::Target *Target,
                                                         const char *TT,
                                                         const char *CPU,
                                                         const char *Features) {
    llvm::TargetOptions Opts;
    llvm::Optional<llvm::Reloc::Model> RM;
    return Target->createTargetMachine(TT, CPU, Features, Opts, RM);
  }

  bool LLVMRustTargetMachineAddPassesToEmitFile(llvm::TargetMachine *TM,
                                                llvm::legacy::PassManager *PM,
                                                llvm::raw_pwrite_stream* Stream,
                                                unsigned CodeGenFileType) {
    return TM->addPassesToEmitFile(*PM, *Stream, (llvm::TargetMachine::CodeGenFileType)CodeGenFileType);
  }
}
