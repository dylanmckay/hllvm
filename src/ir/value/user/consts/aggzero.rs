use SafeWrapper;
use ir::{Constant, Value, Type};
use sys;

/// A constant value of zero of any type.
pub struct ConstantAggregateZero<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantAggregateZero => Constant);

impl<'ctx> ConstantAggregateZero<'ctx>
{
    /// Creates a new zero constant.
    pub fn new(ty: &Type) -> Self {
        unsafe {
            let inner = sys::LLVMRustConstantAggregateZeroGet(ty.inner());
            ConstantAggregateZero(Constant(Value::from_inner(inner)))
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
