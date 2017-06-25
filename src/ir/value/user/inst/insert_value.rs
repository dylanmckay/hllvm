use SafeWrapper;
use ir::{User, Instruction, Value};
use sys;

/// An instruction which can insert an value into an aggregate value.
pub struct InsertValueInst<'ctx>(Instruction<'ctx>);

impl<'ctx> InsertValueInst<'ctx>
{
    /// Creates a new insert value instruction.
    pub fn new(aggregate: &Value,
               new_value: &Value,
               indices: &[u32]) -> Self {
        let indices: Vec<_> = indices.iter().map(|&idx| idx as _).collect();

        unsafe {
            let inner = sys::LLVMRustCreateInsertValueInst(aggregate.inner(), new_value.inner(), &indices);
            wrap_value!(inner => User => Instruction => InsertValueInst)
        }
    }
}

impl_subtype!(InsertValueInst => Instruction);
