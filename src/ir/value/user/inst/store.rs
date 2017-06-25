use SafeWrapper;
use ir::{User, Instruction, Value, AtomicOrdering, SynchronizationScope};
use sys;

const DEFAULT_ALIGN: u32 = 0;
const DEFAULT_ORDERING: AtomicOrdering = AtomicOrdering::NotAtomic;
const DEFAULT_SYNC_SCOPE: SynchronizationScope = SynchronizationScope::CrossThread;

/// A instruction which stores a value to a pointer.
pub struct StoreInst<'ctx>(Instruction<'ctx>);

impl<'ctx> StoreInst<'ctx>
{
    /// Creates a new store instruction.
    pub fn new(value: &Value, ptr: &Value) -> Self {
        StoreInst::new_generic(value, ptr, false, DEFAULT_ALIGN,
                               DEFAULT_ORDERING, DEFAULT_SYNC_SCOPE)
    }

    /// Creates a new volatile store instruction.
    pub fn new_volatile(value: &Value, ptr: &Value) -> Self {
        StoreInst::new_generic(value, ptr, true, DEFAULT_ALIGN,
                               DEFAULT_ORDERING, DEFAULT_SYNC_SCOPE)
    }

    /// Create a new store instruction.
    pub fn new_generic(value: &Value,
                       ptr: &Value,
                       is_volatile: bool,
                       align: u32,
                       ordering: AtomicOrdering,
                       sync_scope: SynchronizationScope) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateStoreInst(value.inner(), ptr.inner(),
                is_volatile, align as _, ordering, sync_scope);
            StoreInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_subtype!(StoreInst => Instruction);
