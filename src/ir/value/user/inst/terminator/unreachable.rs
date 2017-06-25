use SafeWrapper;
use ir::{User, Context, Instruction, TerminatorInst};
use sys;

/// An instruction which cannot be reached.
pub struct UnreachableInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> UnreachableInst<'ctx>
{
    /// Creates a new unreachable instruction.
    pub fn new(context: &Context) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateUnreachableInst(context.inner());
            wrap_value!(inner => User => Instruction => TerminatorInst => UnreachableInst)
        }
    }
}

impl_subtype!(UnreachableInst => TerminatorInst);
