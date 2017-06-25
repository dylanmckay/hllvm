use SafeWrapper;
use ir::{User, Instruction, Value, CatchSwitchInst};
use sys;

/// A catch pad.
pub struct CatchPadInst<'ctx>(Instruction<'ctx>);

impl<'ctx> CatchPadInst<'ctx>
{
    /// Creates a catch pad instruction.
    pub fn new(catch_switch: &CatchSwitchInst,
               arguments: &[&Value]) -> Self {
        let arguments: Vec<_> = arguments.iter().map(|a| a.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustCreateCatchPadInst(catch_switch.inner(), &arguments);
            wrap_value!(inner => User => Instruction => CatchPadInst)
        }
    }
}

impl_subtype!(CatchPadInst => Instruction);
