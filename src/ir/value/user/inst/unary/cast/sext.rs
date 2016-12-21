use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, CastInst, Type};
use sys;

/// Zero-extend an integer.
pub struct SExtInst<'ctx>(CastInst<'ctx>);

impl<'ctx> SExtInst<'ctx>
{
    /// Creates a new SExt instruction.
    pub fn new(value: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateSExtInst(value.inner(), ty.inner());
            SExtInst(CastInst(UnaryInst(Instruction(User(Value::from_inner(inner))))))
        }
    }
}

impl_upcast!(SExtInst => CastInst);
