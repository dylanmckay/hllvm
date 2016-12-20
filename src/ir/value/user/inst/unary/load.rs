use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, AtomicOrdering, SynchronizationScope};
use sys;

const DEFAULT_ALIGN: u32 = 0;
const DEFAULT_ORDERING: AtomicOrdering = AtomicOrdering::NotAtomic;
const DEFAULT_SYNC_SCOPE: SynchronizationScope = SynchronizationScope::CrossThread;

/// A instruction which loads a value from a pointer.
pub struct LoadInst<'ctx>(UnaryInst<'ctx>);

impl<'ctx> LoadInst<'ctx>
{
    /// Creates a new load instruction.
    pub fn new(ptr: &Value) -> Self {
        LoadInst::new_generic(ptr, false, DEFAULT_ALIGN,
                              DEFAULT_ORDERING, DEFAULT_SYNC_SCOPE)
    }

    /// Creates a new volatile load instruction.
    pub fn new_volatile(ptr: &Value) -> Self {
        LoadInst::new_generic(ptr, true, DEFAULT_ALIGN,
                              DEFAULT_ORDERING, DEFAULT_SYNC_SCOPE)
    }

    /// Create a new load instruction.
    pub fn new_generic(ptr: &Value,
                       is_volatile: bool,
                       align: u32,
                       ordering: AtomicOrdering,
                       sync_scope: SynchronizationScope) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateLoadInst(ptr.inner(),
                is_volatile, align as _, ordering, sync_scope);
            LoadInst(UnaryInst(Instruction(User(Value::from_inner(inner)))))
        }
    }
}

impl_upcast!(LoadInst => UnaryInst);
