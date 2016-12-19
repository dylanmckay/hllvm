use {TargetRef, TargetMachineRef, PassManagerRef, RawPWriteStreamRef};
use libc;

cpp! {
    #include "llvm/IR/LegacyPassManager.h"
    #include "llvm/Support/TargetRegistry.h"
    #include "llvm/Target/TargetMachine.h"
    #include "llvm/Target/TargetOptions.h"

    pub fn LLVMRustTargetGetName(target: TargetRef as "llvm::Target*")
        -> *const libc::c_char as "const char*" {
        return target->getName();
    }

    pub fn LLVMRustTargetGetShortDescription(target: TargetRef as "llvm::Target*")
        -> *const libc::c_char as "const char*" {
        return target->getShortDescription();
    }

    pub fn LLVMRustTargetHasMCAsmBackend(target: TargetRef as "llvm::Target*")
        -> bool as "bool" {
        return target->hasMCAsmBackend();
    }

    pub fn LLVMRustTargetHasTargetMachine(target: TargetRef as "llvm::Target*")
        -> bool as "bool" {
        return target->hasTargetMachine();
    }

    pub fn LLVMRustTargetHasJIT(target: TargetRef as "llvm::Target*")
        -> bool as "bool" {
        return target->hasJIT();
    }

    pub fn LLVMRustTargetCreateTargetMachine(target: TargetRef as "llvm::Target*",
                             tt: *const libc::c_char as "const char*",
                             cpu: *const libc::c_char as "const char*",
                             features: *const libc::c_char as "const char*")
        -> TargetMachineRef as "llvm::TargetMachine*" {
        llvm::TargetOptions opts;
        llvm::Optional<llvm::Reloc::Model> rm;
        return target->createTargetMachine(tt, cpu, features, opts, rm);
    }

    pub fn LLVMRustTargetMachineAddPassesToEmitFile(tm: TargetMachineRef as "llvm::TargetMachine*",
                                                    pm: PassManagerRef as "llvm::legacy::PassManager*",
                                                    stream: RawPWriteStreamRef as "llvm::raw_pwrite_stream*",
                                                    file_type: libc::c_uint as "unsigned")
        -> bool as "bool" {
        return tm->addPassesToEmitFile(*pm, *stream, (llvm::TargetMachine::CodeGenFileType)file_type);
    }
}
