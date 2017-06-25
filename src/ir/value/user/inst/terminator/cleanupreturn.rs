use SafeWrapper;
use ir::{User, Instruction, Value, TerminatorInst, CleanupPadInst, Block};
use sys;

pub struct CleanupReturnInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> CleanupReturnInst<'ctx>
{
    /// Creates a new cleanup return instruction.
    pub fn new(cleanup_pad: &CleanupPadInst,
               unwind_block: &Block) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateCleanupReturnInst(cleanup_pad.inner(), unwind_block.inner());
            CleanupReturnInst(TerminatorInst(Instruction(User(Value::from_inner(inner)))))
        }
    }
}

impl_subtype!(CleanupReturnInst => TerminatorInst);
