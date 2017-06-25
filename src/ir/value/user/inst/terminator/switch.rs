use SafeWrapper;
use ir::{User, Instruction, Value, TerminatorInst, Block, ConstantInt};
use sys;

use std::ptr;

/// An switch instruction.
pub struct SwitchInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> SwitchInst<'ctx>
{
    /// Creates a new switch instruction.
    pub fn new(value: &Value,
               default: Option<&Block>) -> Self {
        let default = default.map(|b| b.inner()).unwrap_or(ptr::null_mut());

        unsafe {
            let inner = sys::LLVMRustCreateSwitchInst(value.inner(), default);
            wrap_value!(inner => User => Instruction => TerminatorInst => SwitchInst)
        }
    }

    /// Adds a case to the switch statement.
    pub fn add_case(&mut self,
                    value: &ConstantInt,
                    dest: &Block) {
        unsafe { sys::LLVMRustSwitchInstAddCase(self.inner(), value.inner(), dest.inner()) }
    }
}

impl_subtype!(SwitchInst => TerminatorInst);
