use TargetRef;
use libc;

extern "C" {
    pub fn LLVMRustInitializeAllTargets();

    pub fn LLVMRustTargetRegistryTargets(dest: *mut TargetRef,
                                         max_count: libc::size_t) -> libc::size_t;
}
