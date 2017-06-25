use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, Type};
use sys;

pub struct VAArgInst<'ctx>(UnaryInst<'ctx>);

impl<'ctx> VAArgInst<'ctx>
{
    /// Creates a new `vaarg` instruction.
    pub fn new(list: &Value, ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateVAArgInst(list.inner(), ty.inner());
            wrap_value!(inner => User => Instruction => UnaryInst => VAArgInst)
        }
    }
}

impl_subtype!(VAArgInst => UnaryInst);
