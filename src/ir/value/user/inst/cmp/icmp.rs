use SafeWrapper;
use ir::{User, Instruction, Value, CmpInst, IntegerPredicateKind};
use sys;

/// Integer comparison.
pub struct ICmpInst<'ctx>(CmpInst<'ctx>);

impl<'ctx> ICmpInst<'ctx>
{
    /// Creates a new integer comparision instruction.
    pub fn new(predicate_kind: IntegerPredicateKind,
               lhs: &Value,
               rhs: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateICmpInst(predicate_kind, lhs.inner(), rhs.inner());
            ICmpInst(CmpInst(Instruction(User(Value::from_inner(inner)))))
        }
    }
}

impl_upcast!(ICmpInst => CmpInst);
