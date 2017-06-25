use SafeWrapper;
use ir::{User, Instruction, Value, AtomicOrdering, SynchronizationScope, Context};
use sys;

/// An atomic fence instruction.
pub struct FenceInst<'ctx>(Instruction<'ctx>);

impl<'ctx> FenceInst<'ctx>
{
    /// Creates a new atomic fence instruction.
    pub fn new(context: &Context,
               ordering: AtomicOrdering,
               sync_scope: SynchronizationScope) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateFenceInst(context.inner(), ordering, sync_scope);
            FenceInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_subtype!(FenceInst => Instruction);
