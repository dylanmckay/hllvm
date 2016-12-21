use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, CastInst, Type};
use sys;

/// Zero-extend an integer.
pub struct ZExtInst<'ctx>(CastInst<'ctx>);

impl<'ctx> ZExtInst<'ctx>
{
    /// Creates a new ZExt instruction.
    pub fn new(value: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateZExtInst(value.inner(), ty.inner());
            ZExtInst(CastInst(UnaryInst(Instruction(User(Value::from_inner(inner))))))
        }
    }
}

impl_upcast!(ZExtInst => CastInst);
