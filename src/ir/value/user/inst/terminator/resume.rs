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
            ResumeInst(TerminatorInst(Instruction(User(Value::from_inner(inner)))))
        }
    }
}

impl_subtype!(ResumeInst => TerminatorInst);
