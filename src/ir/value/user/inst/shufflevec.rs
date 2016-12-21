use SafeWrapper;
use ir::{User, Instruction, Value};
use sys;

/// An instruction which can extract an element from a vector.
pub struct ShuffleVectorInst<'ctx>(Instruction<'ctx>);

impl<'ctx> ShuffleVectorInst<'ctx>
{
    /// Creates a new extract element instruction.
    pub fn new(v1: &Value, v2: &Value, mask: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateShuffleVectorInst(v1.inner(), v2.inner(), mask.inner());
            ShuffleVectorInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_upcast!(ShuffleVectorInst => Instruction);
