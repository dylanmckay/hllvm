use SafeWrapper;
use target::FileType;
use support::OutputStream;
use pass;

use sys;

/// An LLVM target machine.
pub struct Machine(sys::TargetMachineRef);

impl Machine
{
    pub fn add_passes_to_emit_file(&self,
                                   pass_manager: &pass::Manager,
                                   stream: &OutputStream,
                                   file_type: FileType) -> bool {
        unsafe {
            sys::LLVMRustTargetMachineAddPassesToEmitFile(self.0,
                                                          pass_manager.inner(),
                                                          stream.inner(),
                                                          file_type)
        }
    }
}

impl SafeWrapper for Machine
{
    type Inner = sys::TargetMachineRef;

    unsafe fn from_inner(inner: sys::TargetMachineRef) -> Self {
        Machine(inner)
    }

    fn inner(&self) -> sys::TargetMachineRef { self.0 }
}
