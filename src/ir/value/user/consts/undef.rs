use SafeWrapper;
use ir::{Constant, Value, Type, User};
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
            UndefValue(Constant(User(Value::from_inner(inner))))
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
