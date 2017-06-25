use SafeWrapper;
use ir::{User, Instruction, Value, UnaryInst, Type};
use sys;

use std::ptr;

/// A stack allocation.
pub struct AllocaInst<'ctx>(UnaryInst<'ctx>);

impl<'ctx> AllocaInst<'ctx>
{
    /// Creates a new stack allocation of a type.
    pub fn new(ty: &Type) -> Self { AllocaInst::new_generic(ty, None) }

    /// Creates a new stack allocation of an array of a type.
    pub fn new_array(ty: &Type,
                     array_size: &Value) -> Self {
        AllocaInst::new_generic(ty, Some(array_size))
    }

    /// Generic alloca construction.
    pub fn new_generic(ty: &Type,
                       array_size: Option<&Value>) -> Self {
        let array_size = array_size.map(SafeWrapper::inner).unwrap_or(ptr::null_mut());

        unsafe {
            let inner = sys::LLVMRustCreateAllocaInst(ty.inner(), array_size);
            AllocaInst(UnaryInst(Instruction(User(Value::from_inner(inner)))))
        }
    }
}

impl_subtype!(AllocaInst => UnaryInst);

#[cfg(test)]
mod test
{
    use ir;

    #[test]
    fn can_create_basic() {
        let context = ir::Context::new();
        let int8 = ir::IntegerType::new(8, &context);

        ir::AllocaInst::new(&int8);
    }

    #[test]
    fn can_create_array() {
        let context = ir::Context::new();
        let int8 = ir::IntegerType::new(8, &context);
        let array_size = ir::ConstantInt::unsigned(&int8, 12);

        ir::AllocaInst::new_array(&int8, &array_size);
    }
}
