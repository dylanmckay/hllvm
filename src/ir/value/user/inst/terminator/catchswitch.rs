use SafeWrapper;
use ir::{User, Instruction, Value, TerminatorInst, Block};
use sys;

pub struct CatchSwitchInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> CatchSwitchInst<'ctx>
{
    /// Creates a new catch return instruction.
    pub fn new(parent_pad: &Value,
               unwind_dest: &Block) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateCatchSwitchInst(parent_pad.inner(), unwind_dest.inner());
            CatchSwitchInst(TerminatorInst(Instruction(User(Value::from_inner(inner)))))
        }
    }

    pub fn add_handler(&mut self, dest: &Block) {
        unsafe { sys::LLVMRustCatchSwitchInstAddHandler(self.inner(), dest.inner()) }
    }
}

impl_subtype!(CatchSwitchInst => TerminatorInst);
