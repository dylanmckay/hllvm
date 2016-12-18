use TargetRef;
use libc;

extern "C" {
    pub fn LLVMRustTargetGetName(_: TargetRef) -> *const libc::c_char;
    pub fn LLVMRustTargetGetShortDescription(_: TargetRef) -> *const libc::c_char;

    pub fn LLVMRustTargetHasMCAsmBackend(_: TargetRef) -> bool;
    pub fn LLVMRustTargetHasTargetMachine(_: TargetRef) -> bool;
    pub fn LLVMRustTargetHasJIT(_: TargetRef) -> bool;
}
