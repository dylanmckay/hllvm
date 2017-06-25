use SafeWrapper;
use ir::{User, Instruction, TerminatorInst, CatchPadInst, Block};
use sys;

pub struct CatchReturnInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> CatchReturnInst<'ctx>
{
    /// Creates a new catch return instruction.
    pub fn new(catch_pad: &CatchPadInst,
               block: &Block) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateCatchReturnInst(catch_pad.inner(), block.inner());
            wrap_value!(inner => User => Instruction => TerminatorInst => CatchReturnInst)
        }
    }
}

impl_subtype!(CatchReturnInst => TerminatorInst);
