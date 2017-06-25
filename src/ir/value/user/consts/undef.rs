use SafeWrapper;
use ir::{Constant, Type, User};
use sys;

/// An undefined value.
pub struct UndefValue<'ctx>(Constant<'ctx>);
impl_subtype!(UndefValue => Constant);

impl<'ctx> UndefValue<'ctx>
{
    /// Creates a new undefined value.
    pub fn new(ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustUndefValueGet(ty.inner());
            wrap_value!(inner => User => Constant => UndefValue)
        }
    }
}

#[cfg(test)]
mod test
{
    use ir;

    #[test]
    fn can_create_undef_i8() {
        let context = ir::Context::new();
        let int8 = ir::IntegerType::new(8, &context);

        ir::UndefValue::new(&int8);
    }
}
