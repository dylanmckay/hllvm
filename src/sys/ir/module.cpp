#include "../llvm.h"

extern "C" {
  llvm::Module *LLVMRustCreateModule(const char *ModuleID,
                                     llvm::LLVMContext *Ctx) {
    return new llvm::Module(llvm::StringRef(ModuleID), *Ctx);
  }

  llvm::Value *LLVMRustModuleGetOrInsertFunction(llvm::Module *Mod,
                                                 const char *Name,
                                                 llvm::Type *Type,
                                                 llvm::Attribute **Attributes,
                                                 size_t AttrCount) {
    llvm::FunctionType *FnTy = llvm::dyn_cast<llvm::FunctionType>(Type);
    assert(FnTy && "must be a function type");

    llvm::AttributeSet AttrSet;

    for (size_t i=0; i<AttrCount; ++i) {
      AttrSet.addAttribute(Type->getContext(), i, *Attributes[i]);
    }

    return Mod->getOrInsertFunction(Name, FnTy, AttrSet);
  }

  void LLVMRustModuleDump(llvm::Module *Mod) {
    Mod->dump();
  }
}
