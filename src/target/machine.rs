use target::FileType;
use support::OutputStream;
use pass;

use sys;

/// An LLVM target machine.
pub struct Machine(sys::TargetMachineRef);

impl Machine
{
    /// Creates a target machine from a pointer to an LLVM `TargetMachine`.
    pub unsafe fn new(inner: sys::TargetMachineRef) -> Self {
        Machine(inner)
    }

    pub fn add_passes_to_emit_file(&self,
                                   pass_manager: &pass::Manager,
                                   stream: &OutputStream,
                                   file_type: FileType) -> bool {
        unsafe {
            sys::LLVMRustTargetMachineAddPassesToEmitFile(self.0,
                                                          pass_manager.inner(),
                                                          stream.inner(),
                                                          file_type as _)
        }
    }
}
