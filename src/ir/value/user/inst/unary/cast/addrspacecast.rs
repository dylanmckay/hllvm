use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, CastInst, Type};
use sys;

/// A address space cast.
pub struct AddrSpaceCastInst<'ctx>(CastInst<'ctx>);

impl<'ctx> AddrSpaceCastInst<'ctx>
{
    /// Creates a new address space cast.
    pub fn new(value: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateAddrSpaceCastInst(value.inner(), ty.inner());
            AddrSpaceCastInst(CastInst(UnaryInst(Instruction(User(Value::from_inner(inner))))))
        }
    }
}

impl_upcast!(AddrSpaceCastInst => CastInst);
