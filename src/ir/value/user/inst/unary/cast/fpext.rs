use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, CastInst, Type};
use sys;

/// Zero-extend an integer.
pub struct FPExtInst<'ctx>(CastInst<'ctx>);

impl<'ctx> FPExtInst<'ctx>
{
    /// Creates a new FPExt instruction.
    pub fn new(value: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateFPExtInst(value.inner(), ty.inner());
            FPExtInst(CastInst(UnaryInst(Instruction(User(Value::from_inner(inner))))))
        }
    }
}

impl_upcast!(FPExtInst => CastInst);
