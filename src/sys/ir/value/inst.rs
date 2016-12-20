use {ContextRef, ValueRef, TypeRef, AtomicOrdering, SynchronizationScope};
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

    pub fn LLVMRustCreateStoreInst(value: ValueRef as "llvm::Value*",
                                   ptr: ValueRef as "llvm::Value*",
                                   is_volatile: bool as "bool",
                                   align: libc::c_uint as "unsigned",
                                   atomic_ordering: AtomicOrdering as "llvm::AtomicOrdering",
                                   sync_scope: SynchronizationScope as "llvm::SynchronizationScope")
        -> ValueRef as "llvm::Value*" {
        return new llvm::StoreInst(value, ptr, is_volatile, align, atomic_ordering, sync_scope);
    }

    pub fn LLVMRustCreateLoadInst(ptr: ValueRef as "llvm::Value*",
                                  is_volatile: bool as "bool",
                                  align: libc::c_uint as "unsigned",
                                  atomic_ordering: AtomicOrdering as "llvm::AtomicOrdering",
                                  sync_scope: SynchronizationScope as "llvm::SynchronizationScope")
        -> ValueRef as "llvm::Value*" {
        return new llvm::LoadInst(ptr, llvm::Twine(), is_volatile, align, atomic_ordering, sync_scope);
    }

    pub fn LLVMRustCreateGetElementPtrInst(pointee_ty: TypeRef as "llvm::Type*",
                                           ptr: ValueRef as "llvm::Value*",
                                           indices: &[ValueRef] as "support::Slice<llvm::Value*>",
                                           in_bounds: bool as "bool")
        -> ValueRef as "llvm::Value*" {

        if (in_bounds)
            return llvm::GetElementPtrInst::CreateInBounds(pointee_ty, ptr, indices.ref());
        else
            return llvm::GetElementPtrInst::Create(pointee_ty, ptr, indices.ref());
    }

    pub fn LLVMRustCreateExtractElementInst(vector: ValueRef as "llvm::Value*",
                                            index: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::ExtractElementInst::Create(vector, index);
    }

    pub fn LLVMRustCreateExtractValueInst(aggregate: ValueRef as "llvm::Value*",
                                          indices: &[libc::c_uint] as "support::Slice<unsigned>")
        -> ValueRef as "llvm::Value*" {
        return llvm::ExtractValueInst::Create(aggregate, indices.ref());
    }

    pub fn LLVMRustCreateInsertElementInst(vector: ValueRef as "llvm::Value*",
                                           new_element: ValueRef as "llvm::Value*",
                                           index: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::InsertElementInst::Create(vector, new_element, index);
    }

    pub fn LLVMRustCreateInsertValueInst(aggregate: ValueRef as "llvm::Value*",
                                         new_value: ValueRef as "llvm::Value*",
                                         indices: &[libc::c_uint] as "support::Slice<unsigned>")
        -> ValueRef as "llvm::Value*" {
        return llvm::InsertValueInst::Create(aggregate, new_value, indices.ref());
    }

    pub fn LLVMRustCreateUnreachableInst(context: ContextRef as "llvm::LLVMContext*")
        -> ValueRef as "llvm::Value*" {
        return new llvm::UnreachableInst(*context);
    }

    pub fn LLVMRustCreateSelectInst(condition: ValueRef as "llvm::Value*",
                                    on_true: ValueRef as "llvm::Value*",
                                    on_false: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::SelectInst::Create(condition, on_true, on_false);
    }

    pub fn LLVMRustCreateAddrSpaceCastInst(value: ValueRef as "llvm::Value*",
                                           ty: TypeRef as "llvm::Type*")
        -> ValueRef as "llvm::Value*" {
        return new llvm::AddrSpaceCastInst(value, ty);
    }

    pub fn LLVMRustCreateFenceInst(context: ContextRef as "llvm::LLVMContext*",
                                   ordering: AtomicOrdering as "unsigned",
                                   sync_scope: SynchronizationScope as "unsigned")
        -> ValueRef as "llvm::Value*" {
        return new llvm::FenceInst(*context, (llvm::AtomicOrdering)ordering,
            (llvm::SynchronizationScope)sync_scope);
    }

    pub fn LLVMRustCreateBinaryOperator(opcode: libc::c_uint as "unsigned",
                                        lhs: ValueRef as "llvm::Value*",
                                        rhs: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::BinaryOperator::Create((llvm::Instruction::BinaryOps)opcode, lhs, rhs);
    }

    pub fn LLVMRustCreateBinaryOperatorNSW(opcode: libc::c_uint as "unsigned",
                                           lhs: ValueRef as "llvm::Value*",
                                           rhs: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::BinaryOperator::CreateNSW((llvm::Instruction::BinaryOps)opcode, lhs, rhs);
    }

    pub fn LLVMRustCreateBinaryOperatorNUW(opcode: libc::c_uint as "unsigned",
                                           lhs: ValueRef as "llvm::Value*",
                                           rhs: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::BinaryOperator::CreateNUW((llvm::Instruction::BinaryOps)opcode, lhs, rhs);
    }

    pub fn LLVMRustCreateBinaryOperatorExact(opcode: libc::c_uint as "unsigned",
                                             lhs: ValueRef as "llvm::Value*",
                                             rhs: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        return llvm::BinaryOperator::CreateExact((llvm::Instruction::BinaryOps)opcode, lhs, rhs);
    }

    pub fn LLVMRustCreatePtrToIntInst(value: ValueRef as "llvm::Value*",
                                      ty: TypeRef as "llvm::Type*")
        -> ValueRef as "llvm::Value*" {
        return new llvm::PtrToIntInst(value, ty);
    }

    pub fn LLVMRustCreateIntToPtrInst(value: ValueRef as "llvm::Value*",
                                      ty: TypeRef as "llvm::Type*")
        -> ValueRef as "llvm::Value*" {
        return new llvm::IntToPtrInst(value, ty);
    }

    // FIXME: add bundle support
    pub fn LLVMRustCreateInvokeInst(func: ValueRef as "llvm::Value*",
                                    args: &[ValueRef] as "support::Slice<llvm::Value*>",
                                    on_success: ValueRef as "llvm::Value*",
                                    on_error: ValueRef as "llvm::Value*")
        -> ValueRef as "llvm::Value*" {
        auto bundles = llvm::None;
        return llvm::InvokeInst::Create(
            support::cast<llvm::Function>(func),
            support::cast<llvm::BasicBlock>(on_success), support::cast<llvm::BasicBlock>(on_error),
            args.ref(), bundles);
    }
}
