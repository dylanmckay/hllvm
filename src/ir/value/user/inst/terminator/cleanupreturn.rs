use SafeWrapper;
use ir::{User, Instruction, TerminatorInst, CleanupPadInst, Block};
use sys;

pub struct CleanupReturnInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> CleanupReturnInst<'ctx>
{
    /// Creates a new cleanup return instruction.
    pub fn new(cleanup_pad: &CleanupPadInst,
               unwind_block: &Block) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateCleanupReturnInst(cleanup_pad.inner(), unwind_block.inner());
            wrap_value!(inner => User => Instruction => TerminatorInst => CleanupReturnInst)
        }
    }
}

impl_subtype!(CleanupReturnInst => TerminatorInst);
