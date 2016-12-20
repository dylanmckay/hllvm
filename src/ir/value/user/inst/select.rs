use SafeWrapper;
use ir::{User, Instruction, Value};
use sys;

/// An instruction which can selects one of two values based on a condition.
pub struct SelectInst<'ctx>(Instruction<'ctx>);

impl<'ctx> SelectInst<'ctx>
{
    /// Creates a new select instruction.
    pub fn new(condition: &Value,
               if_true: &Value,
               if_false: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateSelectInst(condition.inner(),
                if_true.inner(), if_false.inner());
            SelectInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_upcast!(SelectInst => Instruction);
