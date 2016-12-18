use ir::{Context, Value, Constant};
use sys;

pub struct ConstantInt<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantInt => Constant);

impl<'ctx> ConstantInt<'ctx>
{
    /// Gets the integer constant representing boolean 'true'
    pub fn boolean_true(context: &Context) -> Self {
        unsafe {
            let val = sys::LLVMRustConstantIntGetTrue(context.inner());
            ConstantInt(Constant(Value::new(val)))
        }
    }

    /// Gets the integer constant representing boolean 'false'
    pub fn boolean_false(context: &Context) -> Self {
        unsafe {
            let val = sys::LLVMRustConstantIntGetFalse(context.inner());
            ConstantInt(Constant(Value::new(val)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ir::Context;
    use Upcast;

    #[test]
    fn can_get_true_and_false() {
        let context = Context::new();

        assert!(ConstantInt::boolean_true(&context).0.upcast_ref().inner() !=
                ConstantInt::boolean_false(&context).0.upcast_ref().inner());
    }

    #[test]
    fn true_is_eq_to_true() {
        let context = Context::new();

        assert_eq!(ConstantInt::boolean_true(&context).0.upcast_ref().inner(),
                ConstantInt::boolean_true(&context).0.upcast_ref().inner());
    }
}
