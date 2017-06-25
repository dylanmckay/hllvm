use SafeWrapper;
use ir::{Constant, Value, User};
use sys;

pub struct ConstantVector<'ctx>(Constant<'ctx>);
impl_subtype!(ConstantVector => Constant);

impl<'ctx> ConstantVector<'ctx>
{
    pub fn new(values: &[&Value]) -> Self {
        let values: Vec<_> = values.iter().map(|e| e.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustConstantVectorGet(&values);
            wrap_value!(inner => User => Constant => ConstantVector)
        }
    }
}

#[cfg(test)]
mod test
{
    use ir;

    #[test]
    pub fn can_create() {
        let context = ir::Context::new();
        let t = ir::ConstantInt::boolean_true(&context);
        ir::ConstantVector::new(&[&t]);
    }
}
