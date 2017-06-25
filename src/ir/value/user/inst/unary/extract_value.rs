use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst};
use sys;

/// An instruction which can extract an element from an aggregate value.
pub struct ExtractValueInst<'ctx>(UnaryInst<'ctx>);

impl<'ctx> ExtractValueInst<'ctx>
{
    /// Creates a new extract value instruction.
    pub fn new(aggregate: &Value, indices: &[u32]) -> Self {
        let indices: Vec<_> = indices.iter().map(|&idx| idx as _).collect();

        unsafe {
            let inner = sys::LLVMRustCreateExtractValueInst(aggregate.inner(), &indices);
            ExtractValueInst(UnaryInst(Instruction(User(Value::from_inner(inner)))))
        }
    }
}

impl_subtype!(ExtractValueInst => UnaryInst);
