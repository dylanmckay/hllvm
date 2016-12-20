use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, CastInst, Type};
use sys;

/// A cast from a pointer to an integer.
pub struct IntToPtrInst<'ctx>(CastInst<'ctx>);

impl<'ctx> IntToPtrInst<'ctx>
{
    /// Creates a new address space cast.
    pub fn new(value: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateIntToPtrInst(value.inner(), ty.inner());
            IntToPtrInst(CastInst(UnaryInst(Instruction(User(Value::from_inner(inner))))))
        }
    }
}

impl_upcast!(IntToPtrInst => CastInst);
