use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, CastInst, Type};
use sys;

/// Truncate an integer.
pub struct TruncInst<'ctx>(CastInst<'ctx>);

impl<'ctx> TruncInst<'ctx>
{
    /// Creates a new integer truncate instruction.
    pub fn new(value: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateTruncInst(value.inner(), ty.inner());
            TruncInst(CastInst(UnaryInst(Instruction(User(Value::from_inner(inner))))))
        }
    }
}

impl_upcast!(TruncInst => CastInst);
