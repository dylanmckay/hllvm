use SafeWrapper;
use ir::{Context, Type, Constant, User};
use sys;

pub struct ConstantInt<'ctx>(Constant<'ctx>);
impl_subtype!(ConstantInt => Constant);

impl<'ctx> ConstantInt<'ctx>
{
    /// Gets the integer constant representing boolean 'true'
    pub fn boolean_true(context: &Context) -> Self {
        unsafe {
            let val = sys::LLVMRustConstantIntGetTrue(context.inner());
            wrap_value!(val => User => Constant => ConstantInt)
        }
    }

    /// Gets the integer constant representing boolean 'false'
    pub fn boolean_false(context: &Context) -> Self {
        unsafe {
            let val = sys::LLVMRustConstantIntGetFalse(context.inner());
            wrap_value!(val => User => Constant => ConstantInt)
        }
    }

    /// Gets a signed integer of some type.
    pub fn signed(ty: &Type, value: i64) -> Self {
        unsafe {
            let val = sys::LLVMRustConstantIntGetSigned(ty.inner(), value);
            wrap_value!(val => User => Constant => ConstantInt)
        }
    }

    /// Gets an unsigned integer of some type.
    pub fn unsigned(ty: &Type, value: u64) -> Self {
        unsafe {
            let val = sys::LLVMRustConstantIntGetUnsigned(ty.inner(), value);
            wrap_value!(val => User => Constant => ConstantInt)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ir::{Context, IntegerType};
    use Subtype;

    #[test]
    fn can_get_true_and_false() {
        let context = Context::new();

        assert!(ConstantInt::boolean_true(&context).0.upcast_ref().inner() !=
                ConstantInt::boolean_false(&context).0.upcast_ref().inner());
    }

    #[test]
    fn true_is_eq_to_true() {
        let context = Context::new();

        assert_eq!(ConstantInt::boolean_true(&context).inner(),
                   ConstantInt::boolean_true(&context).inner());
    }

    #[test]
    fn true_is_i1_1() {
        let context = Context::new();

        assert_eq!(ConstantInt::boolean_true(&context).inner(),
                   ConstantInt::unsigned(&IntegerType::new(1, &context), 1).inner());
    }
}
