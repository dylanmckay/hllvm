use SafeWrapper;
use ir::{User, Instruction, Value};
use sys;

/// A cleanup pad.
pub struct CleanupPadInst<'ctx>(Instruction<'ctx>);

impl<'ctx> CleanupPadInst<'ctx>
{
    /// Creates a cleanup pad instruction.
    pub fn new(parent_pad: &Value,
               arguments: &[&Value]) -> Self {
        let arguments: Vec<_> = arguments.iter().map(|a| a.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustCreateCleanupPadInst(parent_pad.inner(), &arguments);
            CleanupPadInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_upcast!(CleanupPadInst => Instruction);
