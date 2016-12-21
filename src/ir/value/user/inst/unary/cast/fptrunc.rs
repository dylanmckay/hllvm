use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, CastInst, Type};
use sys;

/// FPTruncate an integer.
pub struct FPTruncInst<'ctx>(CastInst<'ctx>);

impl<'ctx> FPTruncInst<'ctx>
{
    /// Creates a new integer truncate instruction.
    pub fn new(value: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateFPTruncInst(value.inner(), ty.inner());
            FPTruncInst(CastInst(UnaryInst(Instruction(User(Value::from_inner(inner))))))
        }
    }
}

impl_upcast!(FPTruncInst => CastInst);
