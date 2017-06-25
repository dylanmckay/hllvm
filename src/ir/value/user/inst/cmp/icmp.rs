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
            wrap_value!(inner => User => Instruction => CmpInst => ICmpInst)
        }
    }
}

impl_subtype!(ICmpInst => CmpInst);
