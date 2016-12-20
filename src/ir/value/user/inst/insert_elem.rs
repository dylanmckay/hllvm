use SafeWrapper;
use ir::{User, Instruction, Value};
use sys;

/// An instruction which can insert an element into a vector.
pub struct InsertElementInst<'ctx>(Instruction<'ctx>);

impl<'ctx> InsertElementInst<'ctx>
{
    /// Creates a new insert element instruction.
    pub fn new(vector: &Value,
               new_element: &Value,
               index: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateInsertElementInst(vector.inner(), new_element.inner(), index.inner());
            InsertElementInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_upcast!(InsertElementInst => Instruction);
