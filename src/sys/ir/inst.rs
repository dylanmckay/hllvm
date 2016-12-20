use {ContextRef, ValueRef, TypeRef};
use libc;

cpp! {
    #include "support.h"

    #include "llvm/IR/Instructions.h"

    pub fn LLVMRustInstructionInsertAfter(inst: ValueRef as "llvm::Value*",
                                          after: ValueRef as "llvm::Value*") {
        support::cast<llvm::Instruction>(inst)->insertAfter(
            support::cast<llvm::Instruction>(after));
    }

    pub fn LLVMRustInstructionAppend(inst: ValueRef as "llvm::Value*",
                                     block: ValueRef as "llvm::Value*") {
        support::cast<llvm::BasicBlock>(block)->getInstList().push_back(
            support::cast<llvm::Instruction>(inst)
        );
    }

    pub fn LLVMRustCreateReturnInst(context: ContextRef as "llvm::LLVMContext*",
                                    ret_val: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::ReturnInst::Create(*context, ret_val);
    }

    pub fn LLVMRustCreateBranchInst(on_true: ValueRef as "llvm::Value*",
                                    on_false: ValueRef as "llvm::Value*",
                                    condition: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::BranchInst::Create(
            support::cast<llvm::BasicBlock>(on_true), support::cast<llvm::BasicBlock>(on_false),
            condition);
    }

    // FIXME: add bundle support
    pub fn LLVMRustCreateCallInst(func: ValueRef as "llvm::Value*",
                                  args: &[ValueRef] as "support::Slice<llvm::Value*>",
                                  name: *const libc::c_char as "const char*")
        -> ValueRef as "llvm::Value*" {
        auto bundles = llvm::None;
        return llvm::CallInst::Create(
            support::cast<llvm::Function>(func), args.ref(), bundles, name);
    }

    pub fn LLVMRustCreateAllocaInst(ty: TypeRef as "llvm::Type*",
                                    array_size: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return new llvm::AllocaInst(ty, array_size);
    }
}
