use SafeWrapper;
use ir::{User, Instruction, Value, AtomicOrdering, SynchronizationScope};
use sys;

pub struct AtomicCmpXchgInst<'ctx>(Instruction<'ctx>);

impl<'ctx> AtomicCmpXchgInst<'ctx>
{
    pub fn new(pointer: &Value,
               cmp: &Value,
               new_value: &Value,
               success_ordering: AtomicOrdering,
               failure_ordering: AtomicOrdering,
               sync_scope: SynchronizationScope) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateAtomicCmpXchgInst(pointer.inner(), cmp.inner(),
                new_value.inner(), success_ordering, failure_ordering, sync_scope);
            wrap_value!(inner => User => Instruction => AtomicCmpXchgInst)
        }
    }
}

impl_subtype!(AtomicCmpXchgInst => Instruction);
