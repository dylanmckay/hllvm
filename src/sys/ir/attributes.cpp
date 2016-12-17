#include "../llvm.h"

#include <llvm/IR/Attributes.h>

extern "C" {
  llvm::Attribute *LLVMRustAttributeGetInteger(llvm::LLVMContext *Ctx,
                                               unsigned Kind,
                                               uint64_t Val) {
    llvm::Attribute *Attr = new llvm::Attribute();
    *Attr = llvm::Attribute::get(*Ctx, (llvm::Attribute::AttrKind)Kind, Val);
    return Attr;
  }

  llvm::Attribute *LLVMRustAttributeGetString(llvm::LLVMContext *Ctx,
                                              const char *Kind,
                                              const char *Val) {
    llvm::Attribute *Attr = new llvm::Attribute();
    *Attr = llvm::Attribute::get(*Ctx, Kind, Val);
    return Attr;
  }
}
