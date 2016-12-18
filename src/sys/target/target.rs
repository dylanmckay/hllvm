use {TargetRef, TargetMachineRef};
use libc;

extern "C" {
    pub fn LLVMRustTargetGetName(_: TargetRef) -> *const libc::c_char;
    pub fn LLVMRustTargetGetShortDescription(_: TargetRef) -> *const libc::c_char;

    pub fn LLVMRustTargetHasMCAsmBackend(_: TargetRef) -> bool;
    pub fn LLVMRustTargetHasTargetMachine(_: TargetRef) -> bool;
    pub fn LLVMRustTargetHasJIT(_: TargetRef) -> bool;

    pub fn LLVMRustTargetCreateTargetMachine(_: TargetRef,
                             tt: *const libc::c_char,
                             cpu: *const libc::c_char,
                             features: *const libc::c_char) -> TargetMachineRef;
}
