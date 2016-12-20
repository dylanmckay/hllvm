use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, CastInst, Type};
use sys;

/// A cast from a pointer to an integer.
pub struct PtrToIntInst<'ctx>(CastInst<'ctx>);

impl<'ctx> PtrToIntInst<'ctx>
{
    /// Creates a new address space cast.
    pub fn new(value: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreatePtrToIntInst(value.inner(), ty.inner());
            PtrToIntInst(CastInst(UnaryInst(Instruction(User(Value::from_inner(inner))))))
        }
    }
}

impl_upcast!(PtrToIntInst => CastInst);
