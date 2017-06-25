use SafeWrapper;
use ir::{User, Instruction, Value};
use sys;

/// An instruction which can extract an element from a vector.
pub struct ExtractElementInst<'ctx>(Instruction<'ctx>);

impl<'ctx> ExtractElementInst<'ctx>
{
    /// Creates a new extract element instruction.
    pub fn new(vector: &Value, index: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateExtractElementInst(vector.inner(), index.inner());
            ExtractElementInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_subtype!(ExtractElementInst => Instruction);
