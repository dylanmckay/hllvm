use SafeWrapper;
use ir::{User, Instruction, Value, TerminatorInst};
use sys;

/// Resume the propogation of an exception.
pub struct ResumeInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> ResumeInst<'ctx>
{
    /// Creates a new resume instruction.
    pub fn new(exception: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateResumeInst(exception.inner());
            wrap_value!(inner => User => Instruction => TerminatorInst => ResumeInst)
        }
    }
}

impl_subtype!(ResumeInst => TerminatorInst);
