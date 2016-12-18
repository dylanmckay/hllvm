use sys;

/// An LLVM target machine.
pub struct Machine(sys::TargetMachineRef);

impl Machine
{
    /// Creates a target machine from a pointer to an LLVM `TargetMachine`.
    pub unsafe fn new(inner: sys::TargetMachineRef) -> Self {
        Machine(inner)
    }
}
