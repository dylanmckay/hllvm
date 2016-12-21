use SafeWrapper;
use ir::{User, Instruction, Value, CmpInst, FloatPredicateKind};
use sys;

/// Integer comparison.
pub struct FCmpInst<'ctx>(CmpInst<'ctx>);

impl<'ctx> FCmpInst<'ctx>
{
    /// Creates a new integer comparision instruction.
    pub fn new(predicate_kind: FloatPredicateKind,
               lhs: &Value,
               rhs: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateFCmpInst(predicate_kind, lhs.inner(), rhs.inner());
            FCmpInst(CmpInst(Instruction(User(Value::from_inner(inner)))))
        }
    }
}

impl_upcast!(FCmpInst => CmpInst);
