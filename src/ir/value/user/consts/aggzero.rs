use SafeWrapper;
use ir::{Constant, Type, User};
use sys;

/// A constant value of zero of any type.
pub struct ConstantAggregateZero<'ctx>(Constant<'ctx>);
impl_subtype!(ConstantAggregateZero => Constant);

impl<'ctx> ConstantAggregateZero<'ctx>
{
    /// Creates a new zero constant.
    pub fn new(ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustConstantAggregateZeroGet(ty.inner());
            wrap_value!(inner => User => Constant => ConstantAggregateZero)
        }
    }
}

#[cfg(test)]
mod test
{
    use ir;

    #[test]
    fn can_create() {
        let context = ir::Context::new();
        let ty = ir::IntegerType::new(32, &context);
        ir::ConstantAggregateZero::new(&ty);
    }
}
