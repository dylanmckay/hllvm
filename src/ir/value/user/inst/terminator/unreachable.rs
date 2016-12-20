use SafeWrapper;
use ir::{User, Context, Instruction, Value, TerminatorInst};
use sys;

/// An instruction which cannot be reached.
pub struct UnreachableInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> UnreachableInst<'ctx>
{
    /// Creates a new unreachable instruction.
    pub fn new(context: &Context) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateUnreachableInst(context.inner());
            UnreachableInst(TerminatorInst(Instruction(User(Value::from_inner(inner)))))
        }
    }
}

impl_upcast!(UnreachableInst => TerminatorInst);
