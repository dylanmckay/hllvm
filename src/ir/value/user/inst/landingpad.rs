use SafeWrapper;
use ir::{User, Instruction, Value, Type};
use sys;

/// A landing pad.
pub struct LandingPadInst<'ctx>(Instruction<'ctx>);

impl<'ctx> LandingPadInst<'ctx>
{
    /// Creates a landing pad instruction.
    pub fn new(ret_ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateLandingPadInst(ret_ty.inner());
            LandingPadInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_subtype!(LandingPadInst => Instruction);
