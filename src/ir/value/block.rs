use SafeWrapper;
use ir::{Value, Context, Instruction};
use Upcast;
use sys;

/// A basic block.
pub struct Block<'ctx>(Value<'ctx>);

impl<'ctx> Block<'ctx>
{
    /// Creates a new basic block.
    pub fn new(context: &Context) -> Self {
        unsafe {
            Block(Value::from_inner(sys::LLVMRustBasicBlockCreate(context.inner())))
        }
    }

    /// Adds an instruction to a basic block.
    pub fn append(&mut self, inst: &Instruction) {
        unsafe {
            sys::LLVMRustInstructionAppend(inst.upcast_ref().inner(), self.0.inner());
        }
    }
}

impl_upcast!(Block => Value);
